use speedy2d::dimen::Vector2;
use speedy2d::color::Color;
use crate::graphics::Graphics;

crate::create_entity! {
    pub struct Player {
        position: Vector2<f32>,
    }
}
impl Player {
    pub fn new() -> Player {
        //TODO this is dumb
        Player { position: Vector2::new(0., 0.) }
    }
    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.draw_circle(self.position, 10., Color::RED);
    }
    pub fn translate(&mut self, position: Vector2<f32>) {
        self.position = self.position + position;
    }
}