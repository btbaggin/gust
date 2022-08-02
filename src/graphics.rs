use speedy2d::{Graphics2D, shape::Rectangle};
use crate::job_system::ThreadSafeJobQueue;
use std::ops::{Deref, DerefMut};
use speedy2d::image::*;
use speedy2d::{color::Color, font::TextOptions, font::FormattedTextBlock, font::TextLayout};
use std::rc::Rc;
use crate::assets::{Images, Fonts, request_image, request_font};

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

pub struct Label {
    text: String,
    size: f32,
    font: Fonts,
    layout: Option<Rc<FormattedTextBlock>>,
}
impl Label {
    pub fn new(text: String, font: Fonts, size: f32) -> Label {
        Label { text, font, size, layout: None }
    }

    pub fn size(&self) -> f32 { self.size }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        self.layout = None;
    }
    pub fn text(&self) -> &str { &self.text }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.layout = None;
    }

    pub fn render(&mut self, graphics: &mut Graphics, position: (f32, f32), color: Color) {
        if let Some(font) = request_font(graphics, self.font) {
            if let None = self.layout {
                self.layout = Some(font.layout_text(&self.text, self.size, TextOptions::new()));
            }
            graphics.draw_text(position, color, &self.layout.as_ref().unwrap());
        }
    }
}
