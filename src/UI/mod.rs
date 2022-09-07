use crate::V2;
use crate::utils::Rectangle;

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
            HorizontalAlignment::Left => rect.left(),
            HorizontalAlignment::Center => (rect.width() - self.size().x) / 2.,
            HorizontalAlignment::Right => rect.right() - self.size().x,
        }
    }

    fn align_v(&self, rect: &Rectangle, alignment: VerticalAlignment) -> f32 {
        match alignment {
            VerticalAlignment::Top => rect.top(),
            VerticalAlignment::Center => (rect.height() - self.size().y) / 2.,
            VerticalAlignment::Bottom => rect.bottom() - self.size().y,
        }
    }
}

