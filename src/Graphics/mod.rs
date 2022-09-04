use std::ops::{Deref, DerefMut};
use speedy2d::{Graphics2D, shape::Rectangle};
use crate::job_system::ThreadSafeJobQueue;
use glutin::dpi::PhysicalSize;

mod animation;
pub use animation::{AnimationPlayer, SpriteSheetOrientation};

pub struct Graphics<'a> {
    pub graphics: &'a mut Graphics2D,
    pub queue: ThreadSafeJobQueue,
    screen_size: crate::V2,
}
impl<'a> Graphics<'a> {
    pub fn new(graphics: &'a mut Graphics2D, queue: ThreadSafeJobQueue, screen_size: PhysicalSize<u32>) -> Graphics {
        Graphics {
            graphics,
            queue,
            screen_size: crate::V2::new(screen_size.width as f32, screen_size.height as f32)
        }
    }
    pub fn screen_rect(&self) -> speedy2d::shape::Rectangle {
        Rectangle::from_tuples((0., 0.), (self.screen_size.x, self.screen_size.y))
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
