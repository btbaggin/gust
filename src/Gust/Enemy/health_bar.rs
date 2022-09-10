use speedy2d::color::Color;
use crate::utils::{Rectangle, Tween};
use crate::V2;

pub struct HealthBar {
    max: u32,
    target: u32,
    current: Tween<f32>,
}
impl HealthBar {
    pub fn new(max: u32) -> HealthBar {
        HealthBar {
            max,
            target: max,
            current: Tween::new(max as f32, max as f32),
        }
    }
    pub fn set_value(&mut self, current: u32) {
        self.target = current;
        self.current.set_to(current as f32);
    }
    pub fn update(&mut self, delta_time: f32) {
        self.current.lerp(delta_time * 2.);
            //self.current = crate::math::lerp(self.current, self.target as f32, delta_time * 2.);
    }
    pub fn render(&self, position: V2, graphics: &mut crate::Graphics) {
        let width = 50.;
        let left = position.x - width / 2.;
        let top_left = V2::new(left, position.y);
        let current_width = self.current.value() / self.max as f32 * width;//self.current / self.max as f32 * width;
        graphics.draw_rectangle(Rectangle::from_coords(top_left, V2::new(left + width, position.y + 10.)), Color::RED);
        graphics.draw_rectangle(Rectangle::from_coords(top_left, V2::new(left + current_width, position.y + 10.)), Color::GREEN);
    }
}