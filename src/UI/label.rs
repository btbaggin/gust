#![allow(dead_code)]
// use speedy2d::{color::Color, font::TextOptions};
use crate::assets::{Fonts, request_font};
use crate::{V2, Graphics};
use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::{TextLayout, Color};

pub struct Label {
    text: String,
    size: f32,
    font: Fonts,
    layout: RefCell<Option<Rc<TextLayout>>>,
}
impl Label {
    pub fn new(text: String, font: Fonts, size: f32) -> Label {
        Label { text, font, size, layout: RefCell::new(None) }
    }

    pub fn font_size(&self) -> f32 { self.size }
    pub fn set_font_size(&mut self, size: f32) {
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
                *layout = Some(font.layout_text(graphics, &self.text, self.size));
            }
            graphics.draw_text(position, color, layout.as_ref().unwrap());
        }
    }
}
impl super::UiElement for Label {
    fn size(&self) -> V2 {
        let layout = self.layout.borrow();
        match &*layout {
            Some(l) => V2::new(l.size().x, l.size().y),
            None => V2::new(0., 0.),
        }    
    }
}