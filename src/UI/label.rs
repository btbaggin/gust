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
    wrapping: bool,
    cached_size: f32
}
impl Label {
    pub fn new(text: String, font: Fonts, size: f32, color: Color) -> Label {
        Label { text, font, size, color, layout: RefCell::new(None), wrapping: false, cached_size: 0. }
    }
    pub fn wrapping(text: String, font: Fonts, size: f32, color: Color) -> Label {
        Label { text, font, size, color, layout: RefCell::new(None), wrapping: true, cached_size: 0. }
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

    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> Rectangle {
        let mut layout = self.layout.borrow_mut();
        let size = match &*layout {
            Some(l) => l.size(),
            None => if self.wrapping { V2::new(rect.width(), 0.) } else { V2::new(0., 0.) },
        };

        if self.wrapping && self.cached_size != size.x {
            *layout = None;
        }

        let pos = helper.align(rect, &size);
        Rectangle::new(pos, size)
    }

    fn render(&self, graphics: &mut Graphics, rect: &Rectangle) {
        if let Some(font) = request_font(graphics, self.font) {
            let position = rect.top_left();
            let mut layout = self.layout.borrow_mut();
            if layout.is_none() {
                let size = if self.wrapping { rect.width() as i32 } else { i32::MAX };
                *layout = Some(font.layout_text_with_wrap(graphics, &self.text, self.size, size));
            }
            graphics.draw_text(position, self.color, layout.as_ref().unwrap());
        }
    }

    fn update(&mut self, _state: &mut UpdateState, _helper: &mut WidgetHelper, _rect: &Rectangle) { }
}