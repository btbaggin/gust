use crate::graphics::Color;
use crate::utils::{Rectangle, Tween};
use crate::V2;

pub struct HealthBar {
    max: f32,
    current: Tween,
}
impl HealthBar {
    pub fn new(max: f32) -> HealthBar {
        HealthBar {
            max,
            current: Tween::new_slerp(max, max),
        }
    }
    pub fn set_value(&mut self, current: u32) {
        self.current.set_to(current as f32);
    }
    pub fn update(&mut self, delta_time: f32) {
        self.current.update(delta_time);
    }
    pub fn render(&self, position: V2, graphics: &mut crate::Graphics) {
        let width = 50.;
        let left = position.x - width / 2.;
        let top_left = V2::new(left, position.y);
        let current_width = self.current.value() / self.max * width;//self.current / self.max as f32 * width;
        graphics.draw_rectangle(Rectangle::from_coords(top_left, V2::new(left + width, position.y + 10.)), Color::RED);
        graphics.draw_rectangle(Rectangle::from_coords(top_left, V2::new(left + current_width, position.y + 10.)), Color::GREEN);
    }
}