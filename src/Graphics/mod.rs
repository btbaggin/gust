use std::ops::{Deref, DerefMut};
use speedy2d::Graphics2D;
use crate::job_system::ThreadSafeJobQueue;

mod animation;
pub use animation::{AnimationPlayer, SpriteSheetOrientation};

pub struct Graphics<'a> {
    pub graphics: &'a mut Graphics2D,
    pub queue: ThreadSafeJobQueue,
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
