#![feature(maybe_uninit_array_assume_init)]

pub type V2 = cgmath::Vector2<f32>;
pub type V2U = cgmath::Vector2<u32>;

use game_loop::UpdateState;
use speedy2d::color::Color;
use glutin::dpi::PhysicalSize;
use speedy2d::Graphics2D;
use std::sync::Arc;
use std::cell::RefCell;
use logger::LogEntry;
use crate::entity::Scene;

mod assets;
mod job_system;
mod pooled_cache;
mod logger;
mod settings;
mod graphics;
mod utils;
mod entity;
mod game_loop;
mod input;
mod gust;
mod physics;
mod messages;
mod math;
mod ui;
pub use ui::Label;
pub use graphics::Graphics;
use input::Actions;

/* TODO
 * Physics collision layers
 */


struct GameState {
    settings: settings::SettingsFile,
    delta_time_scale: f32,
    audio: std::sync::mpsc::Sender<()>,
    is_playing: bool,
}

impl game_loop::WindowHandler for GameState {
    fn on_render(&mut self, graphics: &mut Graphics, scene_manager: &Scene, _delta_time: f32, _size: PhysicalSize<u32>) {
        graphics.clear_screen(Color::BLACK);

        let entity_manager = crate::entity::entity_manager();
        scene_manager.render(graphics, entity_manager);
    }

    fn on_update(&mut self, state: &mut UpdateState, scene: &mut Scene) -> bool {
        settings::update_settings(&mut self.settings).log("Unable to load new settings");
        state.delta_time *= self.delta_time_scale;
        
        let entity_manager = crate::entity::entity_manager();
        self.is_playing = scene.update(state, entity_manager);

        if state.input.action_pressed(&Actions::Quit) { self.is_playing = false; }
        if state.input.action_pressed(&Actions::Slower) { self.delta_time_scale -= 0.1; }
        if state.input.action_pressed(&Actions::Faster) { self.delta_time_scale += 0.1; }
        self.delta_time_scale = self.delta_time_scale.clamp(0., 1.);
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

    let settings = match settings::load_settings("./settings.txt") {
        Ok(settings) => settings,
        Err(_) => settings::SettingsFile::default()
    };
    let mut input = input::Input::new();
    crate::input::load_input_settings(&mut input, &settings);

    let audio = assets::start_audio_engine();

    let state = GameState {
        delta_time_scale: 1.,
        settings, 
        audio,
        is_playing: true,
    };
    game_loop::create_game_window("gust", false, input, q, Box::new(gust::main_menu::MainMenu::new()), state)
}