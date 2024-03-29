#![feature(maybe_uninit_array_assume_init, new_uninit, let_chains)]

pub type V2 = cgmath::Vector2<f32>;
pub type V2U = cgmath::Vector2<u32>;

use std::sync::Arc;
use std::cell::RefCell;
use logger::LogEntry;
use entity::{Scene, EntityManager};
use input::Actions;
use graphics::Graphics;

mod assets;
mod job_system;
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
mod ui;
mod update_state;
mod generational_array;
pub use update_state::UpdateState;


/* TODO
 * Scene transitions
 * RigidBodies should be offset by half scale
 * Have UI elements handle messages
 */

struct GameState {
    settings: settings::SettingsFile,
    delta_time_scale: f32,
    audio: std::sync::mpsc::Sender<()>,
    is_playing: bool,
}

impl game_loop::WindowHandler for GameState {
    fn on_render(&mut self, graphics: &mut Graphics, root: &crate::ui::Widget, scene: &Scene, entities: &EntityManager) {
        scene.render(graphics, root, entities);
    }

    fn on_update(&mut self, state: &mut UpdateState, root: &mut crate::ui::Widget, scene: &mut Scene) -> bool {
        settings::update_settings(&mut self.settings).log("Unable to load new settings");
        state.delta_time *= self.delta_time_scale;
        
        self.is_playing = scene.update(state, root);

        if state.action_pressed(Actions::Slower) { self.delta_time_scale -= 0.1; }
        if state.action_pressed(Actions::Faster) { self.delta_time_scale += 0.1; }
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
    logger::set_log_level(&settings.get_str(settings::SettingNames::LogLevel));

    let mut input = input::Input::new();
    crate::input::load_input_settings(&mut input, &settings);

    let audio = assets::start_audio_engine();

    let state = GameState {
        delta_time_scale: 1.,
        settings, 
        audio,
        is_playing: true,
    };
    game_loop::start_game_loop("gust", Some((640., 480.)),
                               60, input, q,
                               Box::new(gust::main_menu::MainMenu::new()),
                               state)
}