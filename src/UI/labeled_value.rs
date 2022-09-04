
#![allow(dead_code)]
use speedy2d::{color::Color, font::TextOptions, font::FormattedTextBlock, font::TextLayout};
use crate::assets::{Fonts, request_font};
use crate::{V2, Graphics};
use std::cell::RefCell;
use std::rc::Rc;

pub struct LabeledValue<T: std::fmt::Display + Copy> {
    value: T,
    label: &'static str,
    font: Fonts,
    size: f32,
    layout: RefCell<Option<Rc<FormattedTextBlock>>>,
}
impl<T: std::fmt::Display + Copy> LabeledValue<T> {
    pub fn new(label: &'static str, value: T, font: Fonts, size: f32) -> LabeledValue<T> {
        LabeledValue { value, label, font, size, layout: RefCell::new(None) }
    }

    pub fn value(&self) -> T {
        self.value
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.layout.replace(None);
    }

    pub fn render(&self, graphics: &mut Graphics, position: V2, color: Color) {
        if let Some(font) = request_font(graphics, self.font) {
            let mut layout = self.layout.borrow_mut();
            if layout.is_none() {
                let text = format!("{}: {}", self.label, self.value);
                *layout = Some(font.layout_text(&text, self.size, TextOptions::new()));
            }
            graphics.draw_text(crate::math::from_v2(position), color, layout.as_ref().unwrap());
        }
    }
}
impl<T: std::fmt::Display + Copy> super::UiElement for LabeledValue<T> {
    fn size(&self) -> V2 {
        let layout = self.layout.borrow();
        match &*layout {
            Some(l) => V2::new(l.size().x, l.size().y),
            None => V2::new(0., 0.),
        }    
    }
}