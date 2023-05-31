use crate::V2;
use cgmath::MetricSpace;
use crate::graphics::Color;
use crate::entity::{Entity, EntityInitialization, EntityUpdate};
use crate::messages::{MessageHandler, Messages, MessageContext};
use crate::utils::Rectangle;

pub struct Layout { 
    layout: Vec<V2>,
    lengths: Vec<f32>,
    total_length: f32
}
impl Layout {
    pub fn new(points: &Vec<(u32, u32)>) -> Layout {
        let mut lengths = vec!();

        let grid_size = Layout::grid_size();
        let mut layout = Vec::with_capacity(points.len());
        for p in points {
            layout.push(V2::new(p.0 as f32 * grid_size, p.1 as f32 * grid_size));
        }

        let mut total_length = 0.;
        for i in 0..layout.len() - 1 {
            let length = layout[i].distance(layout[i + 1]);
            lengths.push(length);
            total_length += length;
        }
        Layout { layout, lengths, total_length }
    }

    pub fn get_position(&self, progress: f32) -> V2 {
        let mut pos = self.total_length * progress;

        let mut i = 0;
        while self.lengths[i] < pos {
            pos -= self.lengths[i];
            i += 1;
        }
        let p1 = self.layout[i];
        let p2 = self.layout[i + 1];
        let percent = pos / self.lengths[i];
        p1 + V2::new((p2.x - p1.x) * percent, (p2.y - p1.y) * percent)
    }

    pub fn grid_size() -> f32 {
        crate::graphics::screen_rect().width() / 20.
    }
}
impl crate::entity::EntityBehavior for Layout {
    crate::entity!(Layout);
    
    fn initialize(&mut self, _e: &mut EntityInitialization) { }
    fn update(&mut self, _e: &mut EntityUpdate, _state: &mut crate::UpdateState) { }
    fn render(&self, _e: &Entity, graphics: &mut crate::Graphics) {
        let grid_size = Layout::grid_size();
        let rect = crate::graphics::screen_rect();
        graphics.draw_rectangle(rect, Color::GREEN);

        for i in 0..self.layout.len() - 1 {
            let p1 = self.layout[i];
            let p2 = self.layout[i + 1];

            let rect = Rectangle::new(p1, (p2 - p1) + V2::new(grid_size, grid_size));
            graphics.draw_rectangle(rect, Color::YELLOW);
        }
    }
}
impl MessageHandler for Layout {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages, _context: &mut MessageContext) {}
}