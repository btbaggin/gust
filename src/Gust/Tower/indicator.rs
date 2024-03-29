use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityBehavior};
use crate::messages::{MessageHandler, Messages, MessageContext};
use crate::graphics::Color;

pub struct Indicator { 
    range: f32
}
impl Indicator {
    pub fn new(range: f32) -> Indicator {
        Indicator { range }
    }
}
impl EntityBehavior for Indicator {
    crate::entity!(Indicator);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        e.set_scale(self.range, self.range);
    }

    fn update(&mut self, _e: &mut EntityUpdate, _state: &mut crate::UpdateState) { }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        graphics.draw_circle(e.position, e.scale.x, Color::new(1., 0., 0., 0.25));
    }
}
impl MessageHandler for Indicator {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages, _context: &mut MessageContext) {}
}