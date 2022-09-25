use crate::V2;
use crate::entity::{Entity, EntityInitialization, EntityUpdate, EntityHandle};
use crate::messages::{MessageHandler, Messages};
use crate::utils::Timer;
use cgmath::MetricSpace;

mod bullet;
mod indicator;
pub use bullet::Bullet;
pub use indicator::Indicator;

pub struct Tower { 
    timer: Timer,
    attack_speed: f32,
    damage: f32,
    range: f32,
    target: Option<EntityHandle>,
}
impl Tower {
    pub fn new(attack_speed: f32, damage: f32, range: f32) -> Tower {
        Tower {
            timer: Timer::new(1. / attack_speed),
            attack_speed,
            damage,
            range,
            target: None,
        }
    }
    pub fn attack_speed(&self) -> f32 { self.attack_speed }
    pub fn set_attack_speed(&mut self, attack_speed: f32) {
        self.attack_speed = attack_speed;
        self.timer = Timer::new(1. / attack_speed);
    }
    pub fn damage(&self) -> f32 { self.damage }
    pub fn set_damage(&mut self, damage: f32) { self.damage = damage; }
    pub fn range(&self) -> f32 { self.range }
    pub fn set_range(&mut self, range: f32) { self.range = range; }

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
            let targets = scene.within_distance::<crate::gust::enemy::Enemy>(position, self.range, manager);
            if targets.len() > 0 {
                self.target = Some(targets[0])
            }
        }
    }
}
impl crate::entity::EntityBehavior for Tower {
    crate::entity!(Tower);
    
    fn initialize(&mut self, e: &mut EntityInitialization) {
        let size = crate::gust::level::Layout::grid_size();
        e.set_scale(size, size);
    }

    fn update(&mut self, e: &mut EntityUpdate, state: &mut crate::UpdateState) {
        if self.timer.update(state.delta_time) {
            self.find_target(e.position(), state.entities, state.quad_tree);
            if let Some(t) = self.target {
                let target = state.entities.get(&t).unwrap().position;
                state.entities.create_at(bullet::Bullet::fire(500., self.damage, target), e.position());
            }

        }
    }
    fn render(&self, e: &Entity, graphics: &mut crate::Graphics) {
        self.render_texture(crate::assets::Images::Testing, e, graphics);
    }
}
impl MessageHandler for Tower {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages) {}
}