use crate::{V2, V2U};
use cgmath::MetricSpace;
use speedy2d::shape::Rectangle;
use speedy2d::color::Color;
use crate::entity::{Entity, EntityInitialization, EntityUpdate};
use crate::messages::{MessageHandler, Message};

//TODO tile size based on screen size
const GRID_SIZE: f32 = 32.;

pub struct LevelLayout { 
    layout: Vec<V2>,
    lengths: Vec<f32>,
    total_length: f32
}
impl LevelLayout {
    pub fn new(points: Vec<V2U>) -> LevelLayout {
        let mut lengths = vec!();

        let mut layout = Vec::with_capacity(points.len());
        for p in points {
            layout.push(V2::new(p.x as f32 * GRID_SIZE, p.y as f32 * GRID_SIZE));
        }

        let mut total_length = 0.;
        for i in 0..layout.len() - 1 {
            let length = layout[i].distance(layout[i + 1]);
            lengths.push(length);
            total_length += length;
        }
        LevelLayout { layout, lengths, total_length }
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
}
impl crate::entity::EntityBehavior for LevelLayout {
    crate::entity!(LevelLayout);
    fn initialize(&mut self, _e: &mut EntityInitialization) { }
    fn update(&mut self, _e: &mut EntityUpdate, _state: &mut crate::UpdateState, _scene: &crate::physics::QuadTree) { }
    fn render(&self, _e: &Entity, graphics: &mut crate::Graphics) {
        let size = graphics.screen_size;
        graphics.draw_rectangle(Rectangle::from_tuples((0., 0.), (size.x as f32, size.y as f32)), Color::GREEN);

        for i in 0..self.layout.len() - 1 {
            let p1 = self.layout[i];
            let p2 = self.layout[i + 1];

            let rect = crate::math::sized_rect(p1, (p2 - p1) + V2::new(GRID_SIZE, GRID_SIZE));
            graphics.draw_rectangle(rect, Color::YELLOW);
        }
    }
}
impl MessageHandler for LevelLayout {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}