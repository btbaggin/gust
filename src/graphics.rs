use speedy2d::{Graphics2D, shape::Rectangle};
use crate::job_system::ThreadSafeJobQueue;
use std::ops::{Deref, DerefMut};
use speedy2d::image::*;
use std::rc::Rc;
use crate::assets::{Images, request_image};

pub struct Graphics<'a> {
    pub graphics: &'a mut Graphics2D,
    pub queue: ThreadSafeJobQueue,
}
impl<'a> Deref for Graphics<'a> {
    type Target = Graphics2D;
    fn deref(&self) -> &Self::Target {
        &self.graphics
    }
}
impl<'a> DerefMut for Graphics<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graphics
    }
}

pub struct Texture {
    image: Rc<ImageHandle>,
    bounds: Option<Rectangle>,
}
impl Texture {
    pub fn new(image: Rc<ImageHandle>, bounds: Option<Rectangle>) -> Texture {
        Texture { image, bounds }
    }
    pub fn draw(&self, graphics: &mut Graphics2D, rect: Rectangle) {
        if let Some(b) = &self.bounds {
            graphics.draw_rectangle_image_subset_tinted(rect, speedy2d::color::Color::WHITE, b.clone(), &self.image);
        } else {
            graphics.draw_rectangle_image(rect, &self.image);
        }
    }

    pub fn render(graphics: &mut Graphics, image: Images, rect: Rectangle) {
        if let Some(image) = request_image(graphics, image) {
            image.draw(graphics, rect)
        }
    }
}
impl Deref for Texture {
    type Target = Rc<ImageHandle>;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}
