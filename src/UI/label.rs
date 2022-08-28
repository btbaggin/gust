#![allow(dead_code)]
use speedy2d::{color::Color, font::TextOptions, font::FormattedTextBlock, font::TextLayout};
use crate::assets::{Fonts, request_font};
use crate::Graphics;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Label {
    text: String,
    size: f32,
    font: Fonts,
    layout: RefCell<Option<Rc<FormattedTextBlock>>>,
}
impl Label {
    pub fn new(text: String, font: Fonts, size: f32) -> Label {
        Label { text, font, size, layout: RefCell::new(None) }
    }

    pub fn size(&self) -> f32 { self.size }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        self.layout.replace(None);
    }
    pub fn text(&self) -> &str { &self.text }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.layout.replace(None);
    }

    pub fn render(&self, graphics: &mut Graphics, position: crate::V2, color: Color) {
        if let Some(font) = request_font(graphics, self.font) {
            let mut layout = self.layout.borrow_mut();
            if layout.is_none() {
                *layout = Some(font.layout_text(&self.text, self.size, TextOptions::new()));
            }
            graphics.draw_text(crate::math::from_v2(position), color, layout.as_ref().unwrap());
        }
    }
}
