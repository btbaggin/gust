use speedy2d::color::Color;
use crate::graphics::Graphics;

crate::create_entity! {
    pub struct Player {
        position: crate::V2,
    }
}
impl Player {
    pub fn new() -> Player {
        Player { position: crate::V2::new(0., 0.) }
    }
    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.draw_circle(self.position, 10., Color::RED);
    }
    pub fn translate(&mut self, position: crate::V2) {
        self.position = self.position + position;
    }
}