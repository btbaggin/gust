#![feature(maybe_uninit_array_assume_init)]

//use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::sync::Arc;
use std::cell::RefCell;
use logger::LogEntry;

mod assets;
mod job_system;
mod pooled_cache;
mod logger;
mod settings;
mod graphics;
pub use graphics::{Graphics, Texture};


struct MyWindowHandler {
    queue: job_system::ThreadSafeJobQueue,
    settings: settings::SettingsFile,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut graphics = Graphics { graphics, queue: self.queue.clone() };

        settings::update_settings(&mut self.settings).log("Unable to load new settings");

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        Texture::render(&mut graphics, assets::Images::Testing, speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)));
 
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
        assets::clear_old_cache(&self.settings);
    }

   // If desired, on_mouse_move(), on_key_down(), etc...
}

fn main() {
    logger::init();

    assets::initialize_asset_cache();

    let (queue, _) = job_system::start_job_system();
    let q = Arc::new(std::sync::Mutex::new(RefCell::new(queue)));

    let window = Window::new_centered("gust", (640, 480)).unwrap();
    let settings = match settings::load_settings("./settings.txt") {
        Ok(settings) => settings,
        Err(_) => settings::SettingsFile::default()
    };

    window.run_loop(MyWindowHandler{queue: q, settings});
}
