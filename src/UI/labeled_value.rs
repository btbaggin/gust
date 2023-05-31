
#![allow(dead_code)]
// use speedy2d::{color::Color, font::TextOptions, font::FormattedTextBlock};
use crate::assets::{Fonts, request_font};
use crate::{V2, Graphics, UpdateState};
use std::cell::RefCell;
use std::rc::Rc;
use crate::graphics::{TextLayout, Color};
use crate::ui::WidgetHelper;
use crate::utils::Rectangle;

pub struct LabeledValue<T: std::fmt::Display + Copy> {
    value: T,
    label: &'static str,
    font: Fonts,
    size: f32,
    color: Color,
    layout: RefCell<Option<Rc<TextLayout>>>,
}
impl<T: std::fmt::Display + Copy> LabeledValue<T> {
    pub fn new(label: &'static str, value: T, font: Fonts, size: f32, color: Color) -> LabeledValue<T> {
        LabeledValue { value, label, font, size, color, layout: RefCell::new(None) }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn value(&self) -> T {
        self.value
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.layout.replace(None);
    }

    fn size(&self) -> V2 {
        let layout = self.layout.borrow();
        match &*layout {
            Some(l) => V2::new(l.size().x, l.size().y),
            None => V2::new(0., 0.),
        }    
    }
}
impl<T: std::fmt::Display + Copy + 'static> super::UiElement for LabeledValue<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> Rectangle {
        let layout = self.layout.borrow();
        let size = match &*layout {
            Some(l) => l.size(),
            None => V2::new(0., 0.),
        };

        let pos = helper.align(rect, &size);
        Rectangle::new(pos, size)
    }

    fn render(&self, graphics: &mut Graphics, rect: &Rectangle) {
        if let Some(font) = request_font(graphics, self.font) {
            let position = rect.top_left();
            let mut layout = self.layout.borrow_mut();
            if layout.is_none() {
                let text = format!("{}: {}", self.label, self.value);
                *layout = Some(font.layout_text(graphics, &text, self.size));
            }
            graphics.draw_text(position, self.color, layout.as_ref().unwrap());
        }
    }

    fn update(&mut self, _state: &mut UpdateState, _helper: &mut WidgetHelper, _rect: &Rectangle) { }
}