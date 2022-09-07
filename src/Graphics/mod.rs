use std::ops::{Deref, DerefMut};
use speedy2d::Graphics2D;
use crate::utils::Rectangle;
use crate::job_system::ThreadSafeJobQueue;

mod animation;
pub use animation::{AnimationPlayer, SpriteSheetOrientation};

pub struct Graphics<'a> {
    pub graphics: &'a mut Graphics2D,
    pub queue: ThreadSafeJobQueue,
}
impl<'a> Graphics<'a> {
    pub fn new(graphics: &'a mut Graphics2D, queue: ThreadSafeJobQueue) -> Graphics {
        Graphics {
            graphics,
            queue,
        }
    }
}

impl<'a> Deref for Graphics<'a> {
    type Target = Graphics2D;
    fn deref(&self) -> &Self::Target {
        self.graphics
    }
}
impl<'a> DerefMut for Graphics<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.graphics
    }
}

pub fn screen_rect() -> Rectangle {
    let state = crate::game_loop::global_state();
    Rectangle::new(crate::V2::new(0., 0.), state.screen_size)
}

pub fn on_screen(bounds: &Rectangle) -> bool {
    let rect = screen_rect();
    rect.intersect(&bounds).is_some()
}