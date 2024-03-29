use crate::{V2, UpdateState};
use crate::graphics::Graphics;
use crate::utils::Rectangle;
use crate::logger::PanicLogEntry;

mod label;
mod labeled_value;
mod dock_container;
pub use label::Label;
pub use labeled_value::LabeledValue;
pub use dock_container::DockContainer;

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

pub struct WidgetHelper<'a> {
    pub children: &'a mut Vec<Widget>,
    pending_children: &'a mut Vec<Widget>,
    mark_for_destroy: &'a mut bool,
    h_alignment: &'a HorizontalAlignment,
    v_alignment: &'a VerticalAlignment
}
impl<'a> WidgetHelper<'a> {
    pub fn layout_children(&mut self, rect: &Rectangle) {
        for c in self.children.iter_mut() {
            c.layout(rect)
        }
    }

    pub fn add_child(&mut self, behavior: impl UiElement + 'static) -> &mut Widget {
        let w = Widget::new(behavior, None);
        self.pending_children.push(w);
        self.pending_children.last_mut().unwrap()
    }

    pub fn destroy(&mut self) {
        *self.mark_for_destroy = true;
    }

    pub fn align(&self, rect: &Rectangle, size: &V2) -> V2 {
        let x = match self.h_alignment {
            HorizontalAlignment::Left => rect.left(),
            HorizontalAlignment::Center => (rect.width() - size.x) / 2.,
            HorizontalAlignment::Right => rect.right() - size.x,
        };
        let y = match self.v_alignment {
            VerticalAlignment::Top => rect.top(),
            VerticalAlignment::Center => (rect.height() - size.y) / 2.,
            VerticalAlignment::Bottom => rect.bottom() - size.y,
        };

        rect.top_left() + V2::new(x, y)
    }
}

pub struct Widget {
    bounds: Rectangle,
    h_alignment: HorizontalAlignment,
    v_alignment: VerticalAlignment,
    behavior: Box<dyn UiElement>,
    pending_children: Vec<Widget>,
    children: Vec<Widget>,
    name: Option<&'static str>,
    mark_for_destroy: bool,
}
impl Widget {
    pub fn new(behavior: impl UiElement + 'static, name: Option<&'static str>) -> Widget {
        Widget {
            bounds: Rectangle::new(V2::new(0., 0.), V2::new(0., 0.)),
            h_alignment: HorizontalAlignment::Center,
            v_alignment: VerticalAlignment::Center,
            behavior: Box::new(behavior),
            pending_children: vec!(),
            children: vec!(),
            name,
            mark_for_destroy: false,
        }
    }

    pub fn with_h_alignment(&mut self, alignment: HorizontalAlignment) -> &mut Self {
        self.h_alignment = alignment;
        self
    }

    pub fn with_v_alignment(&mut self, alignment: VerticalAlignment) -> &mut Self {
        self.v_alignment = alignment;
        self
    }

    pub fn add_child(&mut self, behavior: impl UiElement + 'static) -> &mut Self {
        let w = Widget::new(behavior, None);
        self.pending_children.push(w);
        self
    }

    pub fn with_child(&mut self, behavior: impl UiElement + 'static) -> &mut Self {
        let w = Widget::new(behavior, None);
        self.pending_children.push(w);
        self.pending_children.last_mut().unwrap()
    }

    pub fn add_named_child(&mut self, behavior: impl UiElement + 'static, name: &'static str) -> &mut Self {
        let w = Widget::new(behavior, Some(name));
        self.pending_children.push(w);
        self
    }

    pub fn with_named_child(&mut self, behavior: impl UiElement + 'static, name: &'static str) -> &mut Self {
        let w = Widget::new(behavior, Some(name));
        self.pending_children.push(w);
        self.pending_children.last_mut().unwrap()
    }

    pub fn find<T: 'static>(&self, name: &'static str) -> Option<&T> {
        if let Some(n) = self.name && name == n {
            return self.behavior.as_any().downcast_ref::<T>();
        }
        for c in &self.children {
            if let Some(c) = c.find(name) {
                return Some(c);
            }
        }
        None
    }

    pub fn find_mut<T: 'static>(&mut self, name: &'static str) -> Option<&mut T> {
        if let Some(n) = self.name && name == n {
            return self.behavior.as_any_mut().downcast_mut::<T>();
        }
        for c in &mut self.children {
            if let Some(c) = c.find_mut(name) {
                return Some(c);
            }
        }
        None
    }

    pub fn remove(&mut self, name: &'static str) {
        let mut remove_index = None;
        for (i, c) in self.children.iter_mut().enumerate() {
            if let Some(n) = c.name && n == name {
                remove_index = Some(i);
                break;
            }
            c.remove(name);
        }

        if let Some(i) = remove_index {
            self.children.remove(i);
        }
    }

    pub fn update(&mut self, state: &mut UpdateState) {
        for c in &mut self.children {
            c.update(state);
        }

        let mut helper = WidgetHelper {
            children: &mut self.children,
            pending_children: &mut self.pending_children,
            mark_for_destroy: &mut self.mark_for_destroy,
            h_alignment: &self.h_alignment,
            v_alignment: &self.v_alignment,
        };
        self.behavior.update(state, &mut helper, &self.bounds);
    }

    pub fn render(&self, graphics: &mut Graphics, _rect: &Rectangle) {
        let rect = Rectangle::new(self.bounds.top_left(), self.bounds.size());
        self.behavior.render(graphics, &rect);
        for c in &self.children {
            c.render(graphics, &rect);
        }
    }

    pub fn layout(&mut self, rect: &Rectangle) {
        let mut helper = WidgetHelper {
            children: &mut self.children,
            pending_children: &mut self.pending_children,
            mark_for_destroy: &mut self.mark_for_destroy,
            h_alignment: &self.h_alignment,
            v_alignment: &self.v_alignment,
        };

        self.bounds = self.behavior.layout(rect, &mut helper);
    }

    pub fn clear(&mut self) {
        if self.mark_for_destroy {
            self.children.clear();
            return;
        }

        for c in &mut self.children {
            c.clear();
        }
        self.children.retain(|c| !c.mark_for_destroy);
    }

    pub fn children_mut(&mut self) -> &mut Vec<Widget> {
        &mut self.children
    }

    pub fn as_type_mut<T: 'static>(&mut self) -> &mut T {
        self.behavior.as_any_mut().downcast_mut::<T>().log_and_panic()
    }

    pub fn create_widgets(&mut self) {
        for c in self.children.iter_mut() {
            c.create_widgets();
        }
        for w in self.pending_children.drain(..) {
            self.children.push(w);
        }
    }

    pub fn destroy(&mut self) {
        self.mark_for_destroy = true;
    }
}

//TODO need input to turn off if its registered
pub trait UiElement {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn render(&self, graphics: &mut Graphics, bounds: &Rectangle);
    fn update(&mut self, state: &mut UpdateState, helper: &mut WidgetHelper, rect: &Rectangle);
    fn layout(&mut self, rect: &Rectangle, children: &mut WidgetHelper) -> Rectangle;
}

pub struct Root { }
impl UiElement for Root {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn render(&self, _graphics: &mut Graphics, _bounds: &Rectangle) {}
    fn update(&mut self, _state: &mut UpdateState, _helper: &mut WidgetHelper, _rect: &Rectangle) {}
    fn layout(&mut self, rect: &Rectangle, helper: &mut WidgetHelper) -> Rectangle {
        for c in helper.children.iter_mut() {
            c.layout(rect)
        }
        Rectangle::new(V2::new(0., 0.), rect.size())
    }
}
