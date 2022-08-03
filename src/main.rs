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
mod entity;
pub use graphics::{Graphics, Texture, Label};


struct GameState<'a> {
    queue: job_system::ThreadSafeJobQueue,
    settings: settings::SettingsFile,
    sound: Option<&'a mut assets::SoundHandle>,
    entities: entity::EntityManager,
    audio: std::sync::mpsc::Sender<()>,
}

impl WindowHandler for GameState<'_> {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut graphics = Graphics { graphics, queue: self.queue.clone() };

        settings::update_settings(&mut self.settings).log("Unable to load new settings");

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        Texture::render(&mut graphics, assets::Images::Testing, speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)));

        let mut label = Label::new(String::from("testing"), assets::Fonts::Regular, 64.);
        label.render(&mut graphics, (200., 200.), speedy2d::color::Color::RED);
        // Request that we draw another frame once this one has finished

        if let Some(thing) = self.entities.find(&entity::EntityTag::Player) {
            let player = thing.as_any().downcast_ref::<entity::player::Player>().unwrap();
            player.draw(&mut graphics);
        }

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
            },
            Some(VirtualKeyCode::B) => {
                let player = entity::player::Player::new();
                self.entities.create_tagged(Box::new(player), entity::EntityTag::Player);
            },
            Some(VirtualKeyCode::Left) => {
                if let Some(p) = crate::find_entity_mut!(self.entities, entity::EntityTag::Player, entity::player::Player) {
                    p.translate(speedy2d::dimen::Vector2::new(-1., 0.));
                }
            },
            Some(VirtualKeyCode::Right) => {
                if let Some(p) = crate::find_entity_mut!(self.entities, entity::EntityTag::Player, entity::player::Player) {
                    p.translate(speedy2d::dimen::Vector2::new(1., 0.));
                }
            },
            Some(VirtualKeyCode::Up) => {
                if let Some(p) = crate::find_entity_mut!(self.entities, entity::EntityTag::Player, entity::player::Player) {
                    p.translate(speedy2d::dimen::Vector2::new(0., -1.));
                }
            },
            Some(VirtualKeyCode::Down) => {
                if let Some(p) = crate::find_entity_mut!(self.entities, entity::EntityTag::Player, entity::player::Player) {
                    p.translate(speedy2d::dimen::Vector2::new(0., 1.));
                }
            }
            _ => {}
        }
    }

   // If desired, on_mouse_move(), on_key_down(), etc...
}

impl std::ops::Drop for GameState<'_> {
    fn drop(&mut self) {
        self.audio.send(()).log("Failed to send shutdown message to audio thread");
    }
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

    let audio = assets::start_audio_engine();
    window.run_loop(GameState {
        queue: q, 
        settings, 
        sound: None,
        entities: entity::EntityManager::new(),
        audio,
    });
}