use std::sync::atomic::Ordering;
use std::time::Instant;
use std::collections::VecDeque;
use crate::logger::{PanicLogEntry, info};
use crate::assets::AssetSlot;
use crate::job_system::{RawDataPointer, JobType, JobQueue};
use crate::pooled_cache::PooledCacheIndex;
use super::{AssetData, ASSET_STATE_LOADED, AssetTypes, get_slot_mut, get_slot_index, Sounds, ASSET_CACHE};

#[cfg(target_os = "windows")]
mod win32;

// Audio engine taken from: https://github.com/FyroxEngine/Fyrox/tree/master/fyrox-sound
pub const SAMPLE_RATE: u32 = 44100;

#[derive(Debug)]
pub enum SoundError {
    InitializationFailure(String),
}

#[derive(Copy, Clone)]
pub enum SoundStatus {
    Stopped,
    Playing,
    Paused,
}

pub static mut SOUNDS: Option<SoundList> = None;

pub struct SoundList {
    sounds: Vec<Option<SoundHandle>>,
    free_indices: VecDeque<usize>
}
impl SoundList {
    pub fn new() -> SoundList {
        SoundList {
            sounds: Vec::with_capacity(64),
            free_indices: VecDeque::with_capacity(32),
        }
    }
    fn push(&mut self, sound: SoundHandle) -> &mut SoundHandle {
        let inserted_at = if let Some(index) = self.free_indices.pop_front() {
            self.sounds[index] = Some(sound);
            index
        } else {
            self.sounds.push(Some(sound));
            self.sounds.len() - 1
        };
        self.sounds[inserted_at].as_mut().unwrap()
    }

    fn remove(&mut self, index: usize) {
        self.sounds[index] = None;
        self.free_indices.push_back(index);
    }

}

pub struct Sound {
    samples: Vec<f32>,
}

impl Sound {
    pub fn play(queue: &mut JobQueue, sound: Sounds) -> &'static mut SoundHandle {
        // Queue the sound up for loading if it isn't loaded already
        // Since we keep an index to the asset slot in the SoundHandle
        // when it loaded the sound will automatically start playing
        request_sound(queue, sound);
        
        let index = get_slot_index(AssetTypes::Sound(sound));
        let handle = SoundHandle {
            index: index,
            samples_played: 0,
            volume: 1.,
            status: SoundStatus::Playing
        };
        let sounds = unsafe { SOUNDS.as_mut().unwrap() };
        sounds.push(handle)
    }
}
pub struct SoundHandle {
    index: PooledCacheIndex,
    samples_played: usize,
    volume: f32,
    status: SoundStatus
}
impl SoundHandle {
    pub fn status(&self) -> SoundStatus { self.status }
    pub fn set_status(&mut self, status: SoundStatus) { self.status = status; }
    pub fn volume(&self) -> f32 { self.volume }
    pub fn set_volume(&mut self, volume: f32) { self.volume = volume; }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct NativeSample {
    left: i16,
    right: i16,
}

struct MixContext<'a> {
    mix_buffer: &'a mut [(f32, f32)],
    out_data: &'a mut [NativeSample],
}

trait AudioDevice {
    fn get_mixer(&mut self) -> MixContext;
    fn run(&mut self, notice: std::sync::mpsc::Receiver<()>);

    fn mix(&mut self) {
        let context = self.get_mixer();
        // Clear mixer buffer.
        for (left, right) in context.mix_buffer.iter_mut() {
            *left = 0.0;
            *right = 0.0;
        }
    
        // Fill it.
        Self::get_sound_samples(context.mix_buffer);
    
        // Convert to i16 - device expects samples in this format.
        assert_eq!(context.mix_buffer.len(), context.out_data.len());
        for ((left, right), ref mut out_sample) in context.mix_buffer.iter().zip(context.out_data)
        {
            fn sample_to_i16(sample: f32) -> i16 {
                const SCALE: f32 = i16::MAX as f32;
                let clamped = if sample > 1.0 { 1.0 } 
                                else if sample < -1.0 { -1.0 }
                                else { sample };
                (clamped * SCALE) as i16
            }
    
            out_sample.left = sample_to_i16(*left);
            out_sample.right = sample_to_i16(*right);
        }
    }

    fn get_sound_samples(samples: &mut [(f32, f32)]) {
        let sounds = unsafe { SOUNDS.as_mut().unwrap() };
        let assets = unsafe { ASSET_CACHE.as_mut().unwrap() };

        let mut remove_indices = vec!();
        for (i, sound) in sounds.sounds.iter_mut().enumerate() {
            if let Some(handle) = sound {
                match handle.status {
                    SoundStatus::Paused => continue,
                    SoundStatus::Stopped => {
                        remove_indices.push(i);
                    }
                    SoundStatus::Playing => { 
                        if let Some(data) = assets.get_index_mut(handle.index) {
                            if Self::set_samples(data, handle, samples) {
                                remove_indices.push(i);
                            }
                        }
                    }
                }
            }
        }

        for i in remove_indices {
            sounds.remove(i);
        }
    }

    fn set_samples(slot: &mut AssetSlot, handle: &mut SoundHandle, samples: &mut [(f32, f32)]) -> bool {
        if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED &&
            let AssetData::Sound(sound_data) = &slot.data {

            let mut index = handle.samples_played;
            for (left, right) in samples.iter_mut() {
                *left += sound_data.samples[index] * handle.volume;
                *right += sound_data.samples[index + 1] * handle.volume;
                index += 2;

                if index >= sound_data.samples.len() { 
                    return true;
                }
            }
            handle.samples_played = index;
        }
        return false;
    }
}

pub fn start_audio_engine() -> std::sync::mpsc::Sender<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        #[cfg(target_os = "windows")]
        {
            let mut device = win32::DirectSoundDevice::new(SAMPLE_RATE).unwrap();
            device.run(rx)
        }
        #[cfg(not(any(target_os = "windows")))]
        panic!("Sound not implemented on platform");
    });
    tx
}

pub fn request_sound<'a>(queue: &mut JobQueue, sound: Sounds) -> Option<&'a mut Sound> {
    let slot = get_slot_mut(AssetTypes::Sound(sound));
    
    if super::send_job_if_unloaded(queue, slot, JobType::LoadSound(slot.path.clone())) {
        return None;
    }
    
    if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED {
        if let AssetData::Sound(sound) = &mut slot.data {
            slot.last_request = Instant::now();
            return Some(sound);
        }
    }
    None
}

pub fn load_sound_async(path: &'static str, slot: RawDataPointer) {
    info!("Loading sound asynchronously {:?}", path);

    let mut inp_file = std::fs::File::open(std::path::Path::new(path)).log_and_panic();
    let (_header, data) = wav::read(&mut inp_file).log_and_panic();

    let samples = match data {
        wav::BitDepth::Eight(data) => {
            data.iter().map(|i| *i as f32 / i8::MAX as f32).collect()
        },
        wav::BitDepth::Sixteen(data) => {
            data.iter().map(|i| *i as f32 / i16::MAX as f32).collect()
        },
        wav::BitDepth::TwentyFour(data) => {
            const I24_MAX: i32 = 16777215;
            data.iter().map(|i| *i as f32 / I24_MAX as f32).collect()
        },
        wav::BitDepth::ThirtyTwoFloat(data) => data,
        wav::BitDepth::Empty => vec!(),
    };

    let sound = Sound { samples };

    let asset_slot = slot.get_inner::<AssetSlot>();
    asset_slot.size = sound.samples.len() * 4;
    asset_slot.data = AssetData::Sound(sound);
    asset_slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
}