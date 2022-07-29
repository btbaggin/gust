#![feature(maybe_uninit_array_assume_init)]

//use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::sync::Arc;
use std::cell::RefCell;

mod assets;
mod job_system;
mod pooled_cache;
mod logger;

use logger::{LogEntry, PanicLogEntry};

struct MyWindowHandler {
    queue: job_system::ThreadSafeJobQueue,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        let lock = self.queue.lock().log_and_panic();
        let mut queue = lock.borrow_mut();
        if let Some(image) = assets::request_image(graphics, &mut queue, assets::Images::Testing) {
            graphics.draw_rectangle_image(speedy2d::shape::Rectangle::from_tuples((0., 0.), (100., 100.)), image.get_handle());
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

   // If desired, on_mouse_move(), on_key_down(), etc...
}

fn main() {
    logger::init();

    assets::initialize_asset_cache();

    let (queue, _) = job_system::start_job_system();
    let q = Arc::new(std::sync::Mutex::new(RefCell::new(queue)));

    let window = Window::new_centered("gust", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler{queue: q});
}
