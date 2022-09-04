use crate::{Graphics, V2};
use speedy2d::shape::Rectangle;

mod label;
mod labeled_value;
pub use label::Label;
pub use labeled_value::LabeledValue;

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
pub trait UiElement {
    fn size(&self) -> V2;

    fn align_h(&self, rect: &Rectangle, alignment: HorizontalAlignment) -> f32 {
        match alignment {
            HorizontalAlignment::Left => rect.top_left().x,
            HorizontalAlignment::Center => (rect.size().x - self.size().x) / 2.,
            HorizontalAlignment::Right => rect.bottom_right().x - self.size().x,
        }
    }

    fn align_v(&self, rect: &Rectangle, alignment: VerticalAlignment) -> f32 {
        match alignment {
            VerticalAlignment::Top => rect.top_left().y,
            VerticalAlignment::Center => (rect.size().y - self.size().y) / 2.,
            VerticalAlignment::Bottom => rect.bottom_right().y - self.size().y,
        }
    }
}

