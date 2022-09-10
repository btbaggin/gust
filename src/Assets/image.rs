use std::sync::atomic::Ordering;
use std::rc::Rc;
use std::time::Instant;
use crate::utils::Rectangle;
use crate::job_system::{JobType, RawDataPointer};
use crate::logger::{PanicLogEntry, info, warn};
use crate::graphics::Graphics;
use crate::graphics::Texture;
use super::ASSET_STATE_LOADED;
use super::{AssetData, AssetSlot, AssetTypes, get_slot_mut, Images};

        // let data = graphics.create_image_from_file_path(None, ImageSmoothingMode::Linear,"./Assets/packed.png").log_and_panic();
        // let image = Rc::new(data);
        // for tex in read_texture_atlas("./Assets/atlas.tex") {
        //     let image_type = match tex.0.as_str() {
        //         "error.png" => Images::Error,
        //         "question.png" => Images::Question,
        //         "arrow_up.png" => Images::ArrowUp,
        //         "arrow_down.png" => Images::ArrowDown,
        //         "button_a.png" => Images::ButtonA,
        //         "button_b.png" => Images::ButtonB,
        //         "button_x.png" => Images::ButtonX,
        //         "button_y.png" => Images::ButtonY,
        //         "apps.png" => Images::App,
        //         "emulator.png" => Images::Emulator,
        //         "recents.png" => Images::Recent,
        //         "speaker.png" => Images::Speaker,
        //         "settings.png" => Images::Settings,
        //         "everyone.png" => Images::ErsbEveryone,
        //         "everyone10.png" => Images::ErsbEveryone10,
        //         "teen.png" => Images::ErsbTeen,
        //         "mature.png" => Images::ErsbMature,
        //         "adults.png" => Images::ErsbAdultOnly,
        //         _ => panic!("Unknown image found in texture atlas"),
        //     };

        //     let texture = Texture { image: image.clone(), bounds: Some(tex.1) };
        //     map.insert(AssetTypes::Image(image_type), AssetSlot::preloaded("./Assets/packed.png", texture));



// pub fn request_atlas_image<'a>(graphics: &mut Graphics, image: Images) -> Option<&'a Texture> {
//     let slot = get_slot_mut(AssetTypes::PackedImage(image));

//     load_image(graphics, slot, Some(Rectangle::new()))
// }

pub fn request_image<'a>(graphics: &mut Graphics, image: Images) -> Option<&'a Texture> {
    let slot = get_slot_mut(AssetTypes::Image(image));

    load_image(graphics, slot, None)
}

fn load_image<'a>(graphics: &mut Graphics, slot: &'a mut AssetSlot, bounds: Option<Rectangle>) -> Option<&'a Texture> {
    {
        let lock = graphics.queue.lock().log_and_panic();
        let mut queue = lock.borrow_mut();
        if super::send_job_if_unloaded(&mut queue, slot, JobType::LoadImage(slot.path)) {
            return None;
        }
    }

    if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED &&
       let AssetData::RawImage(_) = slot.data {
        
        let data = std::mem::replace(&mut slot.data, AssetData::None);
        if let AssetData::RawImage(data) = data {
            let image = graphics.load_image(data);
            slot.data = AssetData::Image(Texture::new(Rc::new(image), bounds));
        }
    }

    match &slot.data {
        AssetData::Image(image) => {
            slot.last_request = Instant::now();
            Some(image)
        },
        AssetData::None => None,
        _ => panic!("Something is seriously wrong..."),
    }
}

pub fn load_image_async(path: &'static str, slot: RawDataPointer) {
    info!("Loading image asynchronously {:?}", path);

    let path = std::fs::canonicalize(path).expect("invalid image path");
    let data = std::fs::read(&path).log_and_panic();

    let mut reader = image::io::Reader::new(std::io::Cursor::new(data));
    reader = reader.with_guessed_format().log_and_panic();

    match reader.decode() {
        Ok(image) => {
            let buffer = image.into_rgba8();
            let len = buffer.len();
            let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&buffer, buffer.dimensions());
            
            let asset_slot = slot.get_inner::<AssetSlot>();
            asset_slot.size = len;
            asset_slot.data = AssetData::RawImage(raw);
            asset_slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
        },
        Err(e) => warn!("Error loading {:?}: {:?}", path, e),
    }
}

fn read_texture_atlas(path: &str) -> Vec<(String, Rectangle)> {
    use crate::utils::read_type;

    let file = std::fs::read(path).log_and_panic();
    let mut index = 0;
    let total_width = read_type::<i32>(&file, &mut index) as f32;
    let total_height = read_type::<i32>(&file, &mut index) as f32;
    let count = read_type::<i32>(&file, &mut index);

    let mut result = vec!();
    for _ in 0..count {
        let mut name = String::from("");
        loop {
            let c = read_type::<u8>(&file, &mut index);
            if c == 0 || index >= file.len() { break; }

            name.push(c as char);
        }

        let image_width = read_type::<i32>(&file, &mut index);
        let image_height = read_type::<i32>(&file, &mut index);
        let x = read_type::<i32>(&file, &mut index);
        let y = read_type::<i32>(&file, &mut index);

        let width = (x + image_width) as f32 / total_width;
        let height = (y + image_height) as f32 / total_height;
        let x = x as f32 / total_width;
        let y = y as f32 / total_height;
        result.push((name, Rectangle::from_coords(crate::V2::new(x, y), crate::V2::new(width, height))));
    }

    result
}