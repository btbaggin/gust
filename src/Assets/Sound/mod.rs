#![allow(dead_code)]
use std::sync::atomic::Ordering;
use std::time::Instant;
use crate::logger::{PanicLogEntry, info};
use crate::assets::AssetSlot;
use crate::job_system::{RawDataPointer, JobType, JobQueue};
use crate::pooled_cache::PooledCacheIndex;
use super::{AssetData, ASSET_STATE_LOADED, AssetTypes, get_slot_mut, get_slot_index, Sounds, asset_cache};

mod sound_list;
pub use sound_list::{SoundList, sounds};

#[cfg(target_os = "windows")]
mod win32;

// Audio engine taken from: https://github.com/FyroxEngine/Fyrox/tree/master/fyrox-sound
pub const SAMPLE_RATE: u32 = 44100;

#[derive(Debug)]
pub enum SoundError {
    InitializationFailure(String),
}

#[derive(Copy, Clone, PartialEq)]
pub enum SoundStatus {
    Stopped,
    Playing,
    Paused,
    Looping,
}

pub struct Sound {
    samples: Vec<f32>,
}
impl Sound {
    pub fn play(queue: &crate::job_system::ThreadSafeJobQueue, sound: Sounds) -> SoundHandle {
        let handle = PlayingSound::new(queue, sound, SoundStatus::Playing);
        let sounds = sounds();
        sounds.push(handle)
    }

    pub fn repeat(queue: &crate::job_system::ThreadSafeJobQueue, sound: Sounds) -> SoundHandle {
        let handle = PlayingSound::new(queue, sound, SoundStatus::Looping);
        let sounds = sounds();
        sounds.push(handle)
    }

    pub fn get(handle: SoundHandle) -> Option<&'static PlayingSound> { 
        let sounds = sounds();
        sounds.get(handle).as_ref()
    }

    pub fn get_mut(handle: SoundHandle) -> Option<&'static mut PlayingSound> {
        let sounds = sounds();
        sounds.get_mut(handle).as_mut()
    }
}
pub type SoundHandle = usize;

pub struct PlayingSound {
    index: PooledCacheIndex,
    samples_played: usize,
    volume: f32,
    status: SoundStatus
}
impl PlayingSound {
    fn new(queue: &crate::job_system::ThreadSafeJobQueue, sound: Sounds, status: SoundStatus) -> PlayingSound {
        // Queue the sound up for loading if it isn't loaded already
        // Since we keep an index to the asset slot in the PlayingSound
        // when it loaded the sound will automatically start playing
        let lock = queue.lock().log_and_panic();
        let mut queue = lock.borrow_mut();
        request_sound(&mut queue, sound);
        
        let index = get_slot_index(AssetTypes::Sound(sound));
        PlayingSound {
            index: index,
            samples_played: 0,
            volume: 1.,
            status
        }
    }
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
        let sounds = sounds();
        let assets = asset_cache();

        let mut remove_indices = vec!();
        for (i, sound) in sounds.iter_mut().enumerate() {
            if let Some(handle) = sound {
                match handle.status {
                    SoundStatus::Paused => continue,
                    SoundStatus::Stopped => {
                        remove_indices.push(i);
                    }
                    SoundStatus::Playing | SoundStatus::Looping => { 
                        if let Some(data) = assets.get_index_mut(handle.index) {
                            if Self::set_samples(data, handle, samples, handle.status == SoundStatus::Looping) {
                                remove_indices.push(i);
                            }
                        }
                    },
                }
            }
        }

        for i in remove_indices {
            sounds.remove(i);
        }
    }

    fn set_samples(slot: &mut AssetSlot, handle: &mut PlayingSound, samples: &mut [(f32, f32)], looping: bool) -> bool {
        if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED &&
            let AssetData::Sound(sound_data) = &slot.data {

            let mut index = handle.samples_played;
            for (left, right) in samples.iter_mut() {
                *left += sound_data.samples[index] * handle.volume;
                *right += sound_data.samples[index + 1] * handle.volume;
                index += 2;

                if index >= sound_data.samples.len() { 
                    if looping {
                        index = 0;
                    } else {
                        return true;
                    }
                }
            }
            handle.samples_played = index;
        }
        false
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
    
    if super::send_job_if_unloaded(queue, slot, JobType::LoadSound(slot.path)) {
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

    let path = std::fs::canonicalize(path).expect("invalid sound file path");
    let mut inp_file = std::fs::File::open(&path).log_and_panic();
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