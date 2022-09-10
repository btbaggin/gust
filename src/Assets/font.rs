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

    // if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED {
    //     if let AssetData::Raw(data) = &slot.data {
    //         let font = Font::new(data);// TODO .log_and_panic();
    //         slot.data = AssetData::Font(font);
    //     }
    // }

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
    // let data = std::fs::read(&path).log_and_panic();

    let asset_slot = slot.get_inner::<AssetSlot>();
    // super::load_data(asset_slot, data);
    asset_slot.size = 100;//TODO data.len();
    asset_slot.data = AssetData::Font(Font::new(path));
    asset_slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
}
