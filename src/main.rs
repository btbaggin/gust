#![feature(maybe_uninit_array_assume_init)]
#![feature(let_chains)]

pub type V2 = cgmath::Vector2<f32>;
pub type V2U = cgmath::Vector2<u32>;

use speedy2d::color::Color;
use glutin::dpi::PhysicalSize;
use speedy2d::Graphics2D;
use std::sync::Arc;
use std::cell::RefCell;
use logger::LogEntry;
use crate::entity::{SceneBehavior, SceneManager};

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
mod physics;
pub use graphics::{Graphics, Label};
use assets::{Texture, Images, Fonts};
use input::Actions;

/*
 * Message bus
 * Handle for entities
 * Custom V2 class
 */


struct GameState<'a> {
    queue: job_system::ThreadSafeJobQueue,
    settings: settings::SettingsFile,
    sound: Option<&'a mut assets::SoundHandle>,
    audio: std::sync::mpsc::Sender<()>,
    scene: Option<Box<dyn SceneBehavior>>,
    is_playing: bool,
}

impl windowing::WindowHandler for GameState<'_> {
    fn next_scene(&mut self) -> Option<Box<dyn SceneBehavior>> {
        std::mem::replace(&mut self.scene, None)
    }

    fn on_render(&mut self, graphics: &mut Graphics2D, scene_manager: &SceneManager, _delta_time: f32, _size: PhysicalSize<u32>) {
        let mut graphics = Graphics { graphics, queue: self.queue.clone() };
        graphics.clear_screen(Color::BLACK);

        scene_manager.render(&mut graphics);

        Texture::render(&mut graphics, Images::Testing, speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)));

        let mut label = Label::new(String::from("testing"), Fonts::Regular, 64.);
        label.render(&mut graphics, (200., 200.), speedy2d::color::Color::RED);
    }

    fn on_update(&mut self, delta_time: f32, input: &input::Input, scene_manager: &mut SceneManager) -> bool {
        settings::update_settings(&mut self.settings).log("Unable to load new settings");
        scene_manager.update(delta_time, input);        

        if input.action_pressed(&Actions::Quit) { self.is_playing = false; }
        self.is_playing
    }

    fn on_frame_end(&mut self) {
        assets::clear_old_cache(&self.settings);
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

    let state = GameState {
        queue: q.clone(), 
        settings, 
        sound: None,
        audio,
        scene: Some(Box::new(gust::MainLevel::new())),
        is_playing: true,
    };
    windowing::create_game_window("gust", false, input, q.clone(), state)
}