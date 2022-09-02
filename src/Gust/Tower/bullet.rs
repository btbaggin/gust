use crate::V2;
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityBehavior};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};
use crate::messages::{MessageHandler, Message};
use crate::gust::PhysicsLayers;
use crate::math::from_v2;
use cgmath::InnerSpace;

pub struct Bullet { 
    speed: f32,
    damage: u32,
    vector: V2,
}
impl Bullet {
    pub fn fire(speed: f32, damage: u32, target: V2) -> Bullet {
        Bullet {
            speed,
            damage,
            vector: target,
        }
    }
    pub fn damage(&self) -> u32 {
        self.damage
    }
}
impl EntityBehavior for Bullet {
    crate::entity!(Bullet);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        e.set_scale(10., 10.)
         .attach_rigid_body(PhysicsMaterial::METAL, CollisionShape::Circle(Circle::new(10.)))
         .collision_layer(PhysicsLayers::Bullet)
         .collides_with(PhysicsLayers::Enemy);
        self.vector = (self.vector - e.position()).normalize() * self.speed;
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        e.alter_position(self.vector * state.delta_time);
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        graphics.draw_circle(from_v2(e.position), e.scale.x, speedy2d::color::Color::WHITE);
    }
    fn on_collision(&mut self, e: &mut EntityUpdate, other: &Box<dyn EntityBehavior>, messages: &mut crate::messages::MessageBus) {
        e.destroy();
    }
}
impl MessageHandler for Bullet {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}