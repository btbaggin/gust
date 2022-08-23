use crate::input::Actions;
use crate::V2;
use crate::entity::{Entity, EntityHelper};
use crate::physics::{PhysicsMaterial, Polygon, CollisionShape};
use crate::messages::{MessageHandler, Message, MessageBus};

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
        //let shape = CollisionShape::Circle(Circle::new(10.));
        let shape = CollisionShape::Polygon(Polygon::rectangle(50., 50., V2::new(0., 0.)));
        e.set_position(200., 200.)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .set_scale(50., 50.)
         .set_rotation(0.);
    }

    fn update(&mut self, e: &mut EntityHelper, state: &mut crate::game_loop::UpdateState) {
        // if input.action_down(&Actions::Left) { e.alter_position(V2::new(-100. * delta_time, 0.)); }
        // if input.action_down(&Actions::Right) { e.alter_position(V2::new(100. * delta_time, 0.)); }
        // if input.action_down(&Actions::Up) { e.alter_position(V2::new(0., -100. * delta_time)); }
        // if input.action_down(&Actions::Down) { e.alter_position(V2::new(0., 100. * delta_time)); }
        if state.input.action_down(&Actions::Left) { e.apply_force(-100., 0.); }
        if state.input.action_down(&Actions::Right) { e.apply_force(100., 0.); }
        if state.input.action_down(&Actions::Up) { e.apply_force(0., -100.); }
        if state.input.action_down(&Actions::Down) { e.apply_force(0., 100.); }
    }
    
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.render_texture(crate::assets::Images::Testing, e, graphics);
        //TODO add wrappers for graphics

//        graphics.draw_circle(from_v2(e.position), e.scale.x, Color::RED);
    }
}
impl MessageHandler for Player {
    crate::set_address!(Player);
    fn process(&mut self, message: &Message, message_bus: &mut MessageBus) {}
}