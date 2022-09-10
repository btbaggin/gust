use std::sync::atomic::Ordering;
use crate::logger::{PanicLogEntry, info};
use std::time::Instant;
use crate::Graphics;
use crate::job_system::{RawDataPointer, JobType};
use super::{AssetTypes, AssetSlot, AssetData};
use super::{ASSET_STATE_LOADED, get_slot_mut, Fonts};
use crate::graphics::Font;

pub fn request_font<'a>(graphics: &mut Graphics, font: Fonts) -> Option<&'a Font> {
    let slot = get_slot_mut(AssetTypes::Font(font));

    let lock = graphics.queue.lock().log_and_panic();
    let mut queue = lock.borrow_mut();
    if super::send_job_if_unloaded(&mut queue, slot, JobType::LoadFont(slot.path)) {
        return None;
    }

    match &slot.data {
        AssetData::Font(font) => {
            slot.last_request = Instant::now();
            Some(font)
        },
        AssetData::None => None,
        _ => panic!("Something is seriously wrong..."),
    }
}

pub fn load_font_async(path: &'static str, slot: RawDataPointer) {
    info!("Loading font asynchronously {:?}", path);

    let path = std::fs::canonicalize(path).expect("invalid font path");
    let font = Font::new(path);
    // let data = std::fs::read(&path).log_and_panic();

    let asset_slot = slot.get_inner::<AssetSlot>();
    asset_slot.size = font.len();
    asset_slot.data = AssetData::Font(font);
    asset_slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
}
