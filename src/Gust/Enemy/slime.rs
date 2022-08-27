use std::hash::Hash;
use crate::{V2U, V2};
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityHandle};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};
use crate::messages::{MessageHandler, Message, MessageKind};
use crate::graphics::{AnimationPlayer, SpriteSheetOrientation};
use crate::assets::Images;
use crate::gust::PhysicsLayers;

#[derive(Hash, Eq, PartialEq)]
enum SlimeAnimation {
    Idle,
    Walking,
    Hurt,
}

pub struct Slime { 
    initial_pos: V2,
    progress: f32,
    animation: AnimationPlayer<SlimeAnimation>
}
impl Slime {
    pub fn new() -> Slime {
        let mut animation = AnimationPlayer::new(Images::Slime, SpriteSheetOrientation::Vertical);
        animation.add(SlimeAnimation::Idle, 2, 6, V2U::new(36, 32), 0.2);
        animation.add(SlimeAnimation::Walking, 1, 6, V2U::new(36, 32), 0.1);
        animation.add(SlimeAnimation::Hurt, 5, 3, V2U::new(36, 32), 0.5);

        Slime { initial_pos: V2::new(0., 0.), progress: 0., animation }
    }
}
impl crate::entity::EntityBehavior for Slime {
    crate::entity!(Slime);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        let shape = CollisionShape::Circle(Circle::new(35.));
        e.set_scale(75., 75.)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .collision_layer(PhysicsLayers::Enemy as u8)
         .collides_with(PhysicsLayers::Player as u8);
        self.animation.play(SlimeAnimation::Idle);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        let manager = crate::entity::entity_manager();
        let layout = crate::find_entity_mut!(manager, crate::gust::level_layout::LevelLayout).unwrap();

        e.set_position(layout.get_position(self.progress));
        self.animation.update(state);
        self.progress += state.delta_time * 0.1;

        if self.progress >= 1. {
            state.send_message(MessageKind::EnemyGotToEnd);
            e.destroy();
        }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.animation.render(graphics, crate::math::sized_rect(e.position, e.scale));
    }
}
impl MessageHandler for Slime {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}