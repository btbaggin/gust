use crate::V2;
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityHandle};
use crate::messages::{MessageHandler, Message};
use crate::utils::Timer;
use crate::input::Actions;
use cgmath::MetricSpace;

mod bullet;
pub use bullet::Bullet;

enum TowerState {
    Placing,
    Building,
    Placed
}

pub struct Tower { 
    timer: Timer,
    attack_speed: f32,
    damage: u32,
    range: f32,
    target: Option<EntityHandle>,
    state: TowerState
}
impl Tower {
    pub fn new(attack_speed: f32, damage: u32, build_time: f32) -> Tower {
        Tower {
            timer: Timer::new(build_time),
            attack_speed,
            damage,
            range: 100.,
            target: None,
            state: TowerState::Placing
        }
    }

    fn find_target(&mut self, position: V2, manager: &crate::entity::EntityManager, scene: &crate::physics::QuadTree) {
        if self.target.is_some() {
            match manager.get(&self.target.unwrap()) {
                Some(e) => {
                    if e.position.distance2(position) > self.range * self.range {
                        self.target = None;
                    }
                    return;
                },
                None => self.target = None,
            }
        }

        if self.target.is_none() {
            let targets = scene.within_distance(position, 100., manager);
            for t in &targets {
                let entity = manager.get(t).unwrap();
                if crate::utils::entity_as::<crate::gust::enemy::Enemy>(entity).is_some() {
                    self.target = Some(*t);
                    return;
                }
            }
        }
    }
}
impl crate::entity::EntityBehavior for Tower {
    crate::entity!(Tower);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        e.set_scale(75., 75.);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState, scene: &crate::physics::QuadTree) {
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
                    self.find_target(e.position(), state.entities, scene);
                    if let Some(t) = self.target {
                        let target = state.entities.get(&t).unwrap().position;
                        state.entities.create_at(bullet::Bullet::fire(500., self.damage, target), e.position());
                    }

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