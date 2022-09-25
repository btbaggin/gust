#![allow(dead_code)]
// use speedy2d::{color::Color, font::TextOptions};
use crate::assets::{Fonts, request_font};
use crate::{V2, Graphics};
use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::{TextLayout, Color};
use crate::ui::WidgetHelper;
use crate::utils::Rectangle;
use crate::UpdateState;

pub struct Label {
    text: String,
    size: f32,
    font: Fonts,
    color: Color,
    layout: RefCell<Option<Rc<TextLayout>>>,
}
impl Label {
    pub fn new(text: String, font: Fonts, size: f32, color: Color) -> Label {
        Label { text, font, size, color, layout: RefCell::new(None) }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
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
}
impl super::UiElement for Label {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn layout(&mut self, _rect: &Rectangle, _helper: &mut WidgetHelper) -> V2 {
        let layout = self.layout.borrow();
        match &*layout {
            Some(l) => l.size(),
            None => V2::new(0., 0.),
        }  
    }

    fn render(&self, graphics: &mut Graphics, rect: &Rectangle) {
        if let Some(font) = request_font(graphics, self.font) {
            let position = rect.top_left();
            let mut layout = self.layout.borrow_mut();
            if layout.is_none() {
                *layout = Some(font.layout_text(graphics, &self.text, self.size));
            }
            graphics.draw_text(position, self.color, layout.as_ref().unwrap());
        }
    }

    fn update(&mut self, _state: &mut UpdateState, helper: &mut WidgetHelper, _rect: &Rectangle) { }
}