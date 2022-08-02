#![feature(maybe_uninit_array_assume_init)]
#![feature(let_chains)]

//use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, KeyScancode, VirtualKeyCode};
use speedy2d::{Graphics2D, Window};
use std::sync::Arc;
use std::cell::RefCell;
use logger::{LogEntry, PanicLogEntry};

mod assets;
mod job_system;
mod pooled_cache;
mod logger;
mod settings;
mod graphics;
mod utils;
pub use graphics::{Graphics, Texture, Label};


struct MyWindowHandler<'a> {
    queue: job_system::ThreadSafeJobQueue,
    settings: settings::SettingsFile,
    sound: Option<&'a mut assets::SoundHandle>,
}

impl WindowHandler for MyWindowHandler<'_> {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut graphics = Graphics { graphics, queue: self.queue.clone() };

        settings::update_settings(&mut self.settings).log("Unable to load new settings");

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        Texture::render(&mut graphics, assets::Images::Testing, speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)));

        let mut label = Label::new(String::from("testing"), assets::Fonts::Regular, 64.);
        label.render(&mut graphics, (200., 200.), speedy2d::color::Color::RED);
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
        assets::clear_old_cache(&self.settings);
    }

    fn on_key_down(&mut self, _helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        let lock = self.queue.lock().log_and_panic();
        let mut queue = lock.borrow_mut();
        match virtual_key_code {
            Some(VirtualKeyCode::A) => {
                self.sound = Some(assets::Sound::play(&mut queue, assets::Sounds::Piano));
            },
            Some(VirtualKeyCode::Space) => {
                if let Some(handle) = &mut self.sound {
                    match handle.status() {
                        assets::SoundStatus::Playing => handle.set_status(assets::SoundStatus::Paused),
                        assets::SoundStatus::Paused => handle.set_status(assets::SoundStatus::Playing),
                        _ => {}
                    }
                }
            },
            Some(VirtualKeyCode::Tab) => {
                if let Some(handle) = &mut self.sound {
                    handle.set_status(assets::SoundStatus::Stopped);
                }
            }
            _ => {}
        }
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

    assets::start_audio_engine();
    window.run_loop(MyWindowHandler{queue: q, settings, sound: None});
}