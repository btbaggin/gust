use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Instant;
use speedy2d::font::*;
use crate::pooled_cache::{PooledCache, PooledCacheIndex};
use crate::job_system::{JobType, JobQueue};
use crate::logger::PanicLogEntry;

mod image;
mod font;
mod sound;
pub use self::image::{Texture, request_asset_image, request_image, load_image_async};
pub use self::font::{request_font, load_font_async};
pub use self::sound::{start_audio_engine, Sound, PlayingSound, SoundStatus, load_sound_async, SoundHandle};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Images {
    Testing,
    Slime,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Fonts {
    Regular,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Sounds {
    Piano,
}

const ASSET_STATE_UNLOADED: u8 = 0;
const ASSET_STATE_PENDING: u8 = 1;
const ASSET_STATE_LOADED: u8 = 2;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AssetTypes {
    Image(Images),
    Font(Fonts),
    Sound(Sounds),
}

pub enum AssetData {
    Image(Texture),
    Font(Font),
    Sound(Sound),
    Raw(Vec<u8>),
    None,
}

enum SlotTag {
    Dimensions((u32, u32)),
    None,
}

pub struct AssetSlot {
    state: AtomicU8,
    data: AssetData,
    last_request: Instant,
    
    path: &'static str,
    tag: SlotTag,
    size: usize,
}
impl AssetSlot {
    pub fn new(path: &'static str) -> AssetSlot {
        AssetSlot {
            state: AtomicU8::new(ASSET_STATE_UNLOADED),
            path,
            data: AssetData::None,
            tag: SlotTag::None,
            last_request: Instant::now(),
            size: 0,
        }
    }

    fn clear(&mut self) {
        self.state.swap(ASSET_STATE_UNLOADED, Ordering::AcqRel);
        self.data = AssetData::None;
        self.tag = SlotTag::None;
    }
}

crate::singleton!(asset_cache: PooledCache<32, AssetTypes, AssetSlot> = PooledCache::new());

pub fn initialize_asset_cache() {
    let cache = asset_cache();

    cache.insert(AssetTypes::Image(Images::Testing), AssetSlot::new("./resources/1bnn3.jpg"));
    cache.insert(AssetTypes::Font(Fonts::Regular), AssetSlot::new("./resources/font.ttf"));
    cache.insert(AssetTypes::Sound(Sounds::Piano), AssetSlot::new("./resources/piano.wav"));
    cache.insert(AssetTypes::Image(Images::Slime), AssetSlot::new("./resources/slime.png"));
}

fn get_slot_mut(t: AssetTypes) -> &'static mut AssetSlot {
    let slot = asset_cache().get_mut(&t);
    slot.log_message_and_panic(&format!("Invalid slot request {:?}, check that asset has been added", t))
}
fn get_slot_index(t: AssetTypes) -> PooledCacheIndex {
    asset_cache().index_of(&t)
}

fn send_job_if_unloaded(queue: &mut JobQueue, slot: &mut AssetSlot, job: JobType) -> bool {
    if slot.state.load(Ordering::Acquire) == ASSET_STATE_UNLOADED {
        if let Ok(ASSET_STATE_UNLOADED) = slot.state.compare_exchange(ASSET_STATE_UNLOADED, ASSET_STATE_PENDING, Ordering::Acquire, Ordering::Relaxed) {
            queue.send_with_state(job, slot);
            return true;
        }
    }
    false
}

fn load_data(slot: &mut AssetSlot, data: Vec<u8>) {
    slot.size = data.len();
    slot.data = AssetData::Raw(data);
    slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
}

pub fn clear_old_cache(settings: &crate::settings::SettingsFile) {
    let map = asset_cache();

    let mut total_memory = 0;
    let mut last_used_index: Option<PooledCacheIndex> = None;
    let mut last_request = Instant::now();
    let indices = map.indices().collect::<Vec<PooledCacheIndex>>();
    for index in indices {
        let slot = map.get_index_mut(index).unwrap();
        if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED {
            total_memory += slot.size;

            //Find oldest asset
            if slot.last_request < last_request {
                last_request = slot.last_request;
                last_used_index = Some(index);
            } else if slot.last_request.elapsed().as_secs() > 60 {
                //If it hasnt been requested in a minute, remove it regardless
                slot.clear();
            }
        }
    }
    //Remove oldest asset if we are over our memory threshold
    //This will happen once per frame until we are under the threshold
    if total_memory > 1024 * 1024 * settings.get_i32(crate::settings::SettingNames::AssetCacheSizeMb) as usize {
        let slot = map.get_index_mut(last_used_index.unwrap()).unwrap();
        slot.clear();
    }
}