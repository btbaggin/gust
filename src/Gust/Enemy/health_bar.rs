use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use crate::V2;

pub struct HealthBar {
    max: u32,
    target: u32,
    current: f32,
}
impl HealthBar {
    pub fn new(max: u32) -> HealthBar {
        HealthBar {
            max,
            target: max,
            current: max as f32
        }
    }
    pub fn set_value(&mut self, current: u32) {
        self.target = current;
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.current as u32 != self.target {
            self.current = crate::math::lerp(self.current, self.target as f32, delta_time * 2.);
        }
    }
    pub fn render(&self, position: V2, graphics: &mut crate::Graphics) {
        let width = 50.;
        let left = position.x - width / 2.;
        let current_width = self.current / self.max as f32 * width;
        graphics.draw_rectangle(Rectangle::from_tuples((left, position.y), (left + width, position.y + 10.)), Color::RED);
        graphics.draw_rectangle(Rectangle::from_tuples((left, position.y), (left + current_width, position.y + 10.)), Color::GREEN);
    }
}