use std::sync::atomic::{AtomicU8, Ordering};
use std::rc::Rc;
use crate::job_system::{JobType, RawDataPointer};
use crate::logger::{PanicLogEntry, info, warn};
use speedy2d::font::*;
use speedy2d::image::*;
use speedy2d::shape::Rectangle;
use crate::graphics::{Graphics, Texture};
use std::time::Instant;
use crate::pooled_cache::PooledCache;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Images {
    Testing
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fonts {
    Regular,
}

const ASSET_STATE_UNLOADED: u8 = 0;
const ASSET_STATE_PENDING: u8 = 1;
const ASSET_STATE_LOADED: u8 = 2;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AssetTypes {
    Image(Images),
    Font(Fonts),
}

pub enum AssetData {
    Image(Texture),
    Font(Font),
}

pub struct AssetSlot {
    state: AtomicU8,
    path: &'static str,
    data: Vec<u8>,
    dimensions: (u32, u32),
    image: Option<AssetData>,
    last_request: Instant,
}
impl AssetSlot {
    pub fn new(path: &'static str) -> AssetSlot {
        AssetSlot {
            state: AtomicU8::new(ASSET_STATE_UNLOADED),
            path,
            data: Vec::with_capacity(0),
            dimensions: (0, 0),
            image: None,
            last_request: Instant::now(),
        }
    }

    pub fn font(path: &'static str) -> AssetSlot {
        let data = std::fs::read(path).log_and_panic();
        let font = speedy2d::font::Font::new(&data).log_and_panic();

        AssetSlot {
            state: AtomicU8::new(ASSET_STATE_LOADED),
            path,
            data: Vec::with_capacity(0),
            dimensions: (0, 0),
            image: Some(AssetData::Font(font)),
            last_request: Instant::now(),
        }
    }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(0);
        self.image = None;
        self.state.swap(ASSET_STATE_UNLOADED, Ordering::AcqRel);
    }
}

static mut ASSET_CACHE: Option<PooledCache<32, AssetTypes, AssetSlot>> = None;

pub fn initialize_asset_cache() {
    let mut cache = PooledCache::new();
    cache.insert(AssetTypes::Image(Images::Testing), AssetSlot::new(r"C:\Users\allex\Pictures\1bnn3.jpg"));
    unsafe { ASSET_CACHE = Some(cache); }
}

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

fn get_slot_mut(t: AssetTypes) -> &'static mut AssetSlot {
    unsafe { ASSET_CACHE.as_mut().unwrap().get_mut(&t).log_message_and_panic("Invalid asset slot request") }
}

pub fn request_image<'a>(piet: &mut Graphics, image: Images) -> Option<&'a Texture> {
    let slot = get_slot_mut(AssetTypes::Image(image));

    request_asset_image(piet, slot)
}

pub fn request_asset_image<'a>(graphics: &mut Graphics, slot: &'a mut AssetSlot) -> Option<&'a Texture> {
    if slot.state.load(Ordering::Acquire) == ASSET_STATE_UNLOADED {
        if let Ok(ASSET_STATE_UNLOADED) = slot.state.compare_exchange(ASSET_STATE_UNLOADED, ASSET_STATE_PENDING, Ordering::Acquire, Ordering::Relaxed) {

            let lock = graphics.queue.lock().log_and_panic();
            let mut queue = lock.borrow_mut();
            queue.send(JobType::LoadImage((slot.path.clone(), RawDataPointer::new(slot))));
            return None;
        }
    }

    if let None = slot.image {
        if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED {
            let image = graphics.create_image_from_raw_pixels(ImageDataType::RGBA, ImageSmoothingMode::Linear, slot.dimensions, &slot.data).log_and_panic();
            slot.image = Some(AssetData::Image(Texture::new(Rc::new(image), None)));
            slot.data = Vec::with_capacity(0);
        }
    }

    if let Some(AssetData::Image(image)) = slot.image.as_ref() {
        slot.last_request = Instant::now();
        Some(image)
    } else {
        None
    }
}

pub fn request_font(font: Fonts) -> &'static Font {
    let slot = get_slot_mut(AssetTypes::Font(font));

    assert_eq!(slot.state.load(Ordering::Acquire), ASSET_STATE_LOADED, "requested preloaded image, but image is not loaded");

    if let None = slot.image {
        //TODO async
        let font = speedy2d::font::Font::new(&slot.data).log_and_panic();
        slot.image = Some(AssetData::Font(font));
    }

    if let Some(AssetData::Font(font)) = slot.image.as_ref() {
        slot.last_request = Instant::now();
        return font;
    }
    panic!("Requested font on a non-font asset slot");
}

pub fn load_image_async(path: &'static str, slot: RawDataPointer) {
    info!("Loading image asynchronously {:?}", path);

    let data = std::fs::read(&path).log_and_panic();

    let mut reader = image::io::Reader::new(std::io::Cursor::new(data.clone()));
    reader = reader.with_guessed_format().log_and_panic();

    match reader.decode() {
        Ok(image) => {
            let buffer = image.into_rgba8();
            let asset_slot = slot.get_inner::<AssetSlot>();
            asset_slot.dimensions = buffer.dimensions();
            asset_slot.data = buffer.into_vec();
            asset_slot.state.swap(ASSET_STATE_LOADED, Ordering::AcqRel);
        },
        Err(e) => warn!("Error loading {:?}: {:?}", path, e),
    }
}

pub fn clear_old_cache(settings: &crate::settings::SettingsFile) {
    use crate::pooled_cache::PooledCacheIndex;
    let map = unsafe { ASSET_CACHE.as_mut().unwrap() };

    let mut total_memory = 0;
    let mut last_used_index: Option<PooledCacheIndex> = None;
    let mut last_request = Instant::now();
    let indices = map.iter().collect::<Vec<PooledCacheIndex>>();
    for index in indices {
        let slot = map.get_index_mut(index).unwrap();
        if slot.state.load(Ordering::Acquire) == ASSET_STATE_LOADED {
            total_memory += slot.data.len();

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

fn read_texture_atlas(path: &str) -> Vec<(String, Rectangle)> {
    use std::convert::TryInto;

    macro_rules! read_type {
        ($ty:ty, $file:expr, $index:expr) => {{
                let size = std::mem::size_of::<$ty>();
                let value = <$ty>::from_le_bytes($file[$index..($index + size)].try_into().unwrap());
                $index += size;
                value
            }};
    }

    let file = std::fs::read(path).log_and_panic();
    let mut index = 0;
    let total_width = read_type!(i32, file, index) as f32;
    let total_height = read_type!(i32, file, index) as f32;
    let count = read_type!(i32, file, index);

    let mut result = vec!();
    for _ in 0..count {
        let mut name = String::from("");
        loop {
            let c = read_type!(u8, file, index);
            if c == 0 || index >= file.len() { break; }

            name.push(c as char);
        }

        let image_width = read_type!(i32, file, index);
        let image_height = read_type!(i32, file, index);
        let x = read_type!(i32, file, index);
        let y = read_type!(i32, file, index);

        let width = (x + image_width) as f32 / total_width;
        let height = (y + image_height) as f32 / total_height;
        let x = x as f32 / total_width;
        let y = y as f32 / total_height;
        result.push((name, Rectangle::from_tuples((x, y), (width, height))));
    }

    result
}