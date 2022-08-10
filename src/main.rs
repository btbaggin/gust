#![feature(maybe_uninit_array_assume_init)]
#![feature(let_chains)]

pub type V2 = speedy2d::dimen::Vector2<f32>;
pub type V2U = speedy2d::dimen::Vector2<u32>;

use speedy2d::color::Color;
use glutin::dpi::PhysicalSize;
use speedy2d::Graphics2D;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use logger::{LogEntry, PanicLogEntry};

mod assets;
mod job_system;
mod pooled_cache;
mod logger;
mod settings;
mod graphics;
mod utils;
mod entity;
mod windowing;
mod input;
mod gust;
pub use graphics::{Graphics, Label};
use crate::entity::EntityTag;
use assets::{SoundHandle, Texture, Sound, Sounds, Images, Fonts};
use input::Actions;

/*
 * Message bus
 * Scenes
 * Handle for entities
 */


struct GameState<'a> {
    queue: job_system::ThreadSafeJobQueue,
    settings: settings::SettingsFile,
    sound: Option<&'a mut assets::SoundHandle>,
    scene_manager: entity::SceneManager,
    audio: std::sync::mpsc::Sender<()>,
    is_playing: bool,
}

impl windowing::WindowHandler for GameState<'_> {
    fn on_start(&mut self) {
        // We want player to persist between levels so 
        let player = gust::Player::new();
        let player = self.scene_manager.create_tagged(player, EntityTag::Player);

        let main = gust::MainLevel::new();
        self.scene_manager.load(Box::new(main));
        self.scene_manager.add_handle_to_scene(player);

        let lock = self.queue.lock().log_and_panic();
        let mut queue = lock.borrow_mut();
        self.sound = Some(Sound::play(&mut queue, Sounds::Piano));
    }

    fn on_render(&mut self, graphics: &mut Graphics2D, _delta_time: f32, _size: PhysicalSize<u32>) {
        let mut graphics = Graphics { graphics, queue: self.queue.clone() };

        graphics.clear_screen(Color::BLACK);
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        self.scene_manager.render(&mut graphics);

        Texture::render(&mut graphics, Images::Testing, speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)));

        let mut label = Label::new(String::from("testing"), Fonts::Regular, 64.);
        label.render(&mut graphics, (200., 200.), speedy2d::color::Color::RED);
    }

    fn on_update(&mut self, delta_time: f32, input: &input::Input) -> bool {
        settings::update_settings(&mut self.settings).log("Unable to load new settings");
        self.scene_manager.update(delta_time, input);        

        if input.action_pressed(&Actions::Quit) { self.is_playing = false; }
        self.is_playing
    }

    fn on_frame_end(&mut self) {
        assets::clear_old_cache(&self.settings);
        self.scene_manager.dispose_entities();
    }

    fn on_stop(&mut self) {
        self.audio.send(()).log("Failed to send shutdown message to audio thread");
    }
}

fn main() {
    logger::init();

    assets::initialize_asset_cache();

    let (queue, _) = job_system::start_job_system();
    let q = Arc::new(std::sync::Mutex::new(RefCell::new(queue)));

    let mut settings = match settings::load_settings("./settings.txt") {
        Ok(settings) => settings,
        Err(_) => settings::SettingsFile::default()
    };
    let mut input = input::Input::new();
    crate::input::load_input_settings(&mut input, &mut settings);

    let audio = assets::start_audio_engine();

    windowing::create_game_window("gust", false, input,
    GameState {
        queue: q, 
        settings, 
        sound: None,
        scene_manager: entity::SceneManager::new(),
        audio,
        is_playing: true,
    })
}