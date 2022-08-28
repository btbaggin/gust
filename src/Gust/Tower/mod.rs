use std::hash::Hash;
use crate::{V2U, V2};
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityHandle};
use crate::physics::{PhysicsMaterial, Circle, CollisionShape};
use crate::messages::{MessageHandler, Message, MessageKind};
use crate::graphics::{AnimationPlayer, SpriteSheetOrientation};
use crate::assets::Images;
use crate::utils::Timer;
use crate::input::Actions;
use crate::gust::PhysicsLayers;

mod bullet;

enum TowerState {
    Placing,
    Building,
    Placed
}

pub struct Tower { 
    timer: Timer,
    attack_speed: f32,
    damage: u32,
    state: TowerState
}
impl Tower {
    pub fn new(attack_speed: f32, damage: u32, build_time: f32) -> Tower {
        Tower {
            timer: Timer::new(build_time),
            attack_speed,
            damage,
            state: TowerState::Placing
        }
    }
}
impl crate::entity::EntityBehavior for Tower {
    crate::entity!(Tower);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        e.set_scale(75., 75.);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        let manager = crate::entity::entity_manager();

        match self.state {
            TowerState::Placing => {
                e.set_position(state.mouse_pos());
                if state.action_pressed(&Actions::Place) {
                    self.state = TowerState::Building;
                } else if state.action_pressed(&Actions::Cancel) {
                    e.destroy();
                }
            },
            TowerState::Building => {
                if self.timer.update(state.delta_time) {
                    self.state = TowerState::Placed;
                    self.timer = Timer::new(1. / self.attack_speed);
                }
            },
            TowerState::Placed => {
                if self.timer.update(state.delta_time) {
                    //TODO shoot
                    manager.create_at(bullet::Bullet::fire(1000., self.damage, V2::new(0., 0.)), e.position());
                }
            }
        }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        match self.state {
            TowerState::Placing => self.render_texture_tinted(crate::assets::Images::Testing, speedy2d::color::Color::BLUE, e, graphics),
            TowerState::Building => self.render_texture_tinted(crate::assets::Images::Testing, speedy2d::color::Color::GREEN, e, graphics),
            TowerState::Placed => self.render_texture(crate::assets::Images::Testing, e, graphics),
        }
    }
}
impl MessageHandler for Tower {
    crate::handle_messages!();
    fn process(&mut self, _message: &Message) {}
}