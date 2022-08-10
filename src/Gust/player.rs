use speedy2d::color::Color;
use crate::graphics::Graphics;
use crate::input::{Input, Actions};
use crate::V2;

pub struct Player {
    position: crate::V2,
}
impl Player {
    pub fn new() -> Player {
        Player { position: crate::V2::new(0., 0.) }
    }
    pub fn translate(&mut self, position: crate::V2) {
        self.position = self.position + position;
    }
}
impl crate::entity::Entity for Player {
    crate::entity!();

    fn update(&mut self, delta_time: f32, input: &Input) {
        if input.action_down(&Actions::Left) { self.translate(V2::new(-100. * delta_time, 0.)); }
        if input.action_down(&Actions::Right) { self.translate(V2::new(100. * delta_time, 0.)); }
        if input.action_down(&Actions::Up) { self.translate(V2::new(0., -100. * delta_time)); }
        if input.action_down(&Actions::Down) { self.translate(V2::new(0., 100. * delta_time)); }
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        graphics.draw_circle(self.position, 10., Color::RED);
    }
}