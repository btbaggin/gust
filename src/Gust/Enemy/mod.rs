mod spawner;
mod health_bar;
pub use spawner::{Wave, EnemySpawner};

#[derive(serde::Deserialize, Copy, Clone)]
pub enum EnemyType {
    Slime,
}

use std::hash::Hash;
use crate::{V2U, V2};
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityBehavior};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};
use crate::messages::{MessageHandler, Messages};
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
            health_bar: health_bar::HealthBar::new(health as f32),
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
        let size = crate::gust::level::Layout::grid_size();
        let shape = CollisionShape::Circle(Circle::new(size / 2.));
        e.set_scale(size, size)
         .attach_rigid_body(PhysicsMaterial::METAL, shape)
         .collision_layer(PhysicsLayers::Enemy)
         .collides_with(PhysicsLayers::Player as u8 | PhysicsLayers::Bullet as u8);
        self.animation.play(SlimeAnimation::Idle);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        let layout = state.entities.find_as_mut::<crate::gust::level::Layout>().unwrap();

        e.set_position(layout.get_position(self.progress));
        self.animation.update(state);
        self.health_bar.update(state.delta_time);
        self.progress += state.delta_time * 0.1;

        if self.progress >= 1. {
            state.send_message(Messages::EnemyGotToEnd(10));
            e.destroy();
        }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.animation.render(graphics, crate::utils::Rectangle::new(e.position, e.scale));

        let position = V2::new(e.position.x + e.scale.x / 2., e.position.y);
        self.health_bar.render(position, graphics);
    }
    fn on_collision(&mut self, e: &mut EntityUpdate, other: &Entity, messages: &mut crate::messages::MessageBus) {
        if self.health > 0 &&
            let Some(b) = crate::utils::entity_as::<crate::gust::tower::Bullet>(other) {
           self.take_damage(b.damage());
            if self.health == 0 {
                messages.send(Messages::EnemyKilled);
                e.destroy();
            }
        }
    }
}
impl MessageHandler for Enemy {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages) {}
}