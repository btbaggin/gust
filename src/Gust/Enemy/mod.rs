mod spawner;
mod health_bar;
pub use spawner::{Wave, EnemySpawner};

pub enum EnemyType {
    Slime,
}

use std::hash::Hash;
use crate::V2U;
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityBehavior};
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

pub struct Enemy { 
    health: u32,
    health_bar: health_bar::HealthBar,
    progress: f32,
    animation: AnimationPlayer<SlimeAnimation>
}
impl Enemy {
    pub fn new() -> Enemy {
        let mut animation = AnimationPlayer::new(Images::Slime, SpriteSheetOrientation::Vertical);
        animation.add(SlimeAnimation::Idle, 2, 6, V2U::new(36, 32), 0.2);
        animation.add(SlimeAnimation::Walking, 1, 6, V2U::new(36, 32), 0.1);
        animation.add(SlimeAnimation::Hurt, 5, 3, V2U::new(36, 32), 0.5);

        let health = 100;
        Enemy { 
            health,
            health_bar: health_bar::HealthBar::new(health),
            progress: 0.,
            animation
        }
    }

    fn take_damage(&mut self, damage: u32) {
        self.health -= u32::min(self.health, damage);
        self.health_bar.set_value(self.health);
    }
}
impl EntityBehavior for Enemy {
    crate::entity!(Enemy);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        let shape = CollisionShape::Circle(Circle::new(35.));
        e.set_scale(75., 75.)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .collision_layer(PhysicsLayers::Enemy)
         .collides_with(PhysicsLayers::Player as u8 | PhysicsLayers::Bullet as u8);
        self.animation.play(SlimeAnimation::Idle);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        let manager = crate::entity::entity_manager();
        let layout = crate::find_entity_mut!(manager, crate::gust::level_layout::LevelLayout).unwrap();

        e.set_position(layout.get_position(self.progress));
        self.animation.update(state);
        self.health_bar.update(state.delta_time);
        self.progress += state.delta_time * 0.1;

        if self.progress >= 1. {
            state.send_message(MessageKind::EnemyGotToEnd);
            e.destroy();
        }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.animation.render(graphics, crate::math::sized_rect(e.position, e.scale));
        self.health_bar.render(e.position, graphics);
    }
    fn on_collision(&mut self, e: &mut EntityUpdate, other: &Box<dyn EntityBehavior>, messages: &mut crate::messages::MessageBus) {
        if let Some(b) = crate::entity_as!(other, crate::gust::tower::Bullet) {
           self.take_damage(b.damage());
            if self.health == 0 {
                messages.send(MessageKind::EnemyKilled);
                e.destroy();
            }
        }
    }
}
impl MessageHandler for Enemy {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}