use crate::utils::Rectangle;
use crate::UpdateState;
use crate::ui::WidgetHelper;
use crate::graphics::{Graphics, Color};
use crate::V2;

enum DockDirection {
    Horizontal,
    Vertical,
}

pub struct DockContainer {
    width: f32,
    height: f32,
    direction: DockDirection,
    background: Option<Color>,
}
impl DockContainer {
    pub fn horizontal(width: f32, height: f32, background: Option<Color>) -> DockContainer {
        DockContainer {
            width, height, background, direction: DockDirection::Horizontal
        }
    }
    pub fn vertical(width: f32, height: f32, background: Option<Color>) -> DockContainer {
        DockContainer {
            width, height, background, direction: DockDirection::Vertical
        }
    }
}
impl super::UiElement for DockContainer {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    
    fn render(&self, graphics: &mut Graphics, bounds: &Rectangle) {
        if let Some(b) = self.background {
            graphics.draw_rectangle(bounds.clone(), b);
        }
    }
    fn update(&mut self, _state: &mut UpdateState, helper: &mut WidgetHelper, _rect: &Rectangle) { }
    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> V2 {
        let width = rect.width() * self.width;
        let height = rect.height() * self.height;

        let mut offset = 0.;
        let child_count = helper.children.len();
        
        for c in helper.children.iter_mut() {
            offset += match self.direction {
                DockDirection::Horizontal => {
                    let child_rect = Rectangle::new(V2::new(rect.left() + offset, rect.top()), V2::new(width / child_count as f32, height));
                    c.layout(&child_rect);
                    c.bounds.width()
                },
                DockDirection::Vertical => {
                    let child_rect = Rectangle::new(V2::new(rect.left(), rect.top() + offset), V2::new(width, height / child_count as f32));
                    c.layout(&child_rect);
                    c.bounds.height()
                }
            }

        }

        V2::new(width, height)
    }
}