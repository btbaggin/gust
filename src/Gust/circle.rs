use speedy2d::color::Color;
use crate::V2;
use crate::input::Input;
use crate::entity::{Entity, EntityHelper, EntityManager};
use crate::physics::{PhysicsMaterial, Polygon, CollisionShape};
use crate::messages::{MessageHandler, Message, MessageBus};

pub struct Circle { }
impl Circle {
    pub fn new() -> Circle {
        Circle { }
    }
}
impl crate::entity::EntityBehavior for Circle {
    crate::entity!();
    
    fn initialize(&self, e: &mut EntityHelper) {
        let shape = CollisionShape::Polygon(Polygon::rectangle(75., 75., V2::new(0., 0.)));
        //let shape = CollisionShape::Circle(crate::physics::Circle::new(75.));
        e.set_position(100., 100.)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .set_scale(75., 75.);
    }

    fn update(&mut self, e: &mut EntityHelper, update_state: &mut crate::game_loop::UpdateState) { }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        //TODO add wrappers for graphics
       crate::assets::Texture::render(graphics, crate::assets::Images::Testing, crate::utils::sized_rect(e.position, e.scale));
        // graphics.draw_circle(from_v2(e.position + e.scale / 2.), e.scale.x, Color::RED);
    }
}
impl MessageHandler for Circle {
    crate::set_address!(Circle);
    fn process(&mut self, message: &Message, message_bus: &mut MessageBus) {}
}