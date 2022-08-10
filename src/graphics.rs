use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use speedy2d::{Graphics2D, color::Color, font::TextOptions, font::FormattedTextBlock, font::TextLayout};
use crate::job_system::ThreadSafeJobQueue;
use crate::assets::{Fonts, request_font};

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
