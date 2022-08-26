use std::hash::Hash;
use crate::{V2U, V2};
use crate::entity::{Entity, EntityHelper};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};
use crate::messages::{MessageHandler, Message};
use crate::graphics::{AnimationPlayer, SpriteSheetOrientation};
use crate::assets::Images;

#[derive(Hash, Eq, PartialEq)]
enum SlimeAnimation {
    Idle,
    Walking,
    Hurt,
}

pub struct Slime { 
    initial_pos: V2,
    animation: AnimationPlayer<SlimeAnimation>
}
impl Slime {
    pub fn new(initial_pos: V2) -> Slime {
        let mut animation = AnimationPlayer::new(Images::Slime, SpriteSheetOrientation::Vertical);
        animation.add(SlimeAnimation::Idle, 2, 6, V2U::new(36, 32), 0.2);
        animation.add(SlimeAnimation::Walking, 1, 6, V2U::new(36, 32), 0.2);
        animation.add(SlimeAnimation::Hurt, 5, 3, V2U::new(36, 32), 0.5);
        Slime { initial_pos, animation }
    }
}
impl crate::entity::EntityBehavior for Slime {
    crate::entity!(Slime);
    
    fn initialize(&mut self, e: &mut EntityHelper) {
        let shape = CollisionShape::Circle(Circle::new(75.));
        e.set_position(self.initial_pos.x, self.initial_pos.y)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .set_scale(75., 75.);
        self.animation.play(SlimeAnimation::Idle);
    }

    fn update(&mut self, _e: &mut EntityHelper, state: &mut crate::game_loop::UpdateState) {
        self.animation.update(state);
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.animation.render(graphics, crate::math::sized_rect(e.position, e.scale));
    }
}
impl MessageHandler for Slime {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}