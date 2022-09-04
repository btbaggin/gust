use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions, EntityManager};
use crate::gust::{player::Player, enemy::EnemySpawner, enemy::Wave, enemy::EnemyType};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler, MessageBus};
use crate::assets::{Fonts};
use crate::ui::{LabeledValue, UiElement, HorizontalAlignment};
use crate::{V2U, V2};
use crate::input::Actions;

const MAX_HEALTH: u32 = 100;

enum LevelStatus {
    InProgress,
    Success,
    Fail
}

pub struct Level { 
    spawner: EnemySpawner,
    health: LabeledValue<u32>,
    gold: LabeledValue<u32>,
    status: LevelStatus,
}
impl Level {
    pub fn new() -> Level {
        Level { 
            spawner: EnemySpawner::new(),
            health: LabeledValue::new("Health", MAX_HEALTH, Fonts::Regular, 24.),
            gold: LabeledValue::new("Gold", 0, Fonts::Regular, 24.),
            status: LevelStatus::InProgress,
        }
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, _messages: &mut MessageBus, entities: &mut EntityManager) {    
        let player = Player::new();
        entities.create_options(player, EntityCreationOptions::Tag);

        self.spawner.add_wave(Wave::new(5, 0.5, EnemyType::Slime), 2.);
        self.spawner.add_wave(Wave::new(10, 1., EnemyType::Slime), 5.);

        let positions = vec!(V2U::new(0, 10), V2U::new(10, 10), V2U::new(10, 3), V2U::new(20, 3));
        let layout = crate::gust::level_layout::LevelLayout::new(positions);
        entities.create_options(layout, EntityCreationOptions::Tag);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::UpdateState) -> SceneLoad {
        self.spawner.update(state.delta_time, state.entities);

        if state.action_pressed(&Actions::GetTower) {
            let tower = crate::gust::tower::Tower::new(10., 5, 200.);
            state.entities.create(tower);
        }

        match self.status {
            LevelStatus::InProgress => SceneLoad::None,
            LevelStatus::Success => SceneLoad::Unload,
            LevelStatus::Fail => SceneLoad::Unload,
        }
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        self.health.render(graphics, V2::new(0., 0.), speedy2d::color::Color::WHITE);
        
        let x = self.gold.align_h(&graphics.screen_rect(), HorizontalAlignment::Right);
        self.gold.render(graphics, V2::new(x, 0.), speedy2d::color::Color::WHITE);
    }
}
impl MessageHandler for Level {
    crate::handle_messages!(Messages::EnemyGotToEnd(0), Messages::EnemyKilled);
    
    fn process(&mut self, message: &Messages) { 
        match message {
            Messages::EnemyGotToEnd(damage) => {
                let current = self.health.value();
                let damage = u32::min(*damage, current);
                self.health.set_value(current - damage);

                // This code needs to run unconditionally
                let level_done = self.spawner.mark_enemy_dead();
                if self.health.value() == 0 {
                    self.status = LevelStatus::Fail
                } else if level_done {
                    self.status = LevelStatus::Success

                }
            }
            Messages::EnemyKilled => {
                self.gold.set_value(self.gold.value() + 10);
                if self.spawner.mark_enemy_dead() {
                    self.status = LevelStatus::Success
                }
            }
        }
    }
}