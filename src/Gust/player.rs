use speedy2d::color::Color;
use crate::input::{Input, Actions};
use crate::V2;
use crate::utils::from_v2;
use crate::entity::{Entity, EntityHelper};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};

pub struct Player {
}
impl Player {
    pub fn new() -> Player {
        Player { }
    }
}
impl crate::entity::EntityBehavior for Player {
    crate::entity!();

    fn initialize(&self, e: &mut EntityHelper) {
        let shape = CollisionShape::Circle(Circle::new(10.));
        e.set_position(V2::new(10., 10.))
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .set_scale(V2::new(10., 10.))
         .set_rotation(0.);
    }

    fn update(&mut self, e: &mut EntityHelper, delta_time: f32, input: &Input) {
        if input.action_down(&Actions::Left) { e.alter_position(V2::new(-100. * delta_time, 0.)); }
        if input.action_down(&Actions::Right) { e.alter_position(V2::new(100. * delta_time, 0.)); }
        if input.action_down(&Actions::Up) { e.alter_position(V2::new(0., -100. * delta_time)); }
        if input.action_down(&Actions::Down) { e.alter_position(V2::new(0., 100. * delta_time)); }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        //TODO add wrappers for graphics
        graphics.draw_circle(from_v2(e.position), e.scale.x, Color::RED);
    }
}