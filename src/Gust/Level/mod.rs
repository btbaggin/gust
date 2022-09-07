use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions, EntityManager, EntityHandle};
use crate::gust::{player::Player, enemy::EnemySpawner, enemy::Wave};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler, MessageBus};
use crate::assets::{Fonts};
use crate::ui::{LabeledValue, UiElement, HorizontalAlignment};
use crate::V2;
use crate::input::Actions;
use std::collections::HashMap;

mod layout;
pub use layout::Layout;

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
    indicator: Option<EntityHandle>,
}
impl Level {
    pub fn new() -> Level {
        Level { 
            spawner: EnemySpawner::new(),
            health: LabeledValue::new("Health", MAX_HEALTH, Fonts::Regular, 24.),
            gold: LabeledValue::new("Gold", 0, Fonts::Regular, 24.),
            status: LevelStatus::InProgress,
            indicator: None
        }
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, _messages: &mut MessageBus, entities: &mut EntityManager) {    
        let file = std::fs::read_to_string("./resources/levels.ron").unwrap();
        let v = ron::from_str::<HashMap<&str, crate::gust::game_data::Level>>(&file).unwrap();
        let level = &v["level_1"];

        for w in &level.waves {
            self.spawner.add_wave(Wave::new(w.count, w.interval, w.r#type), w.time);
        }

        let layout = crate::gust::level::Layout::new(&level.layout);
        entities.create_options(layout, EntityCreationOptions::Tag);

        for t in &level.towers {
            let tower = crate::gust::tower::Tower::new(10., 5, 200.);
            entities.create_at(tower, V2::new(t.0 as f32 * 32., t.1 as f32 * 32.));
        }

        let player = Player::new();
        entities.create_options(player, EntityCreationOptions::Tag);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::UpdateState) -> SceneLoad {
        self.spawner.update(state.delta_time, state.entities);

        //Show tower indicator
        if state.action_pressed(&Actions::Select) {
            let entities = state.quad_tree.at_point::<crate::gust::tower::Tower>(state.mouse_pos(), state.entities);
            assert!(entities.len() <= 1);

            if let Some(i) = self.indicator && 
               let Some(e) = state.entities.get_mut(&i) {
                e.destroy();
            }

            if entities.len() == 1 {
                let e = state.entities.get(&entities[0]).unwrap();
                self.indicator = Some(state.entities.create_at(crate::gust::tower::Indicator::new(200.), e.position));
            }
        }

        match self.status {
            LevelStatus::InProgress => SceneLoad::None,
            LevelStatus::Success => SceneLoad::Unload,
            LevelStatus::Fail => SceneLoad::Unload,
        }
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        self.health.render(graphics, V2::new(0., 0.), speedy2d::color::Color::WHITE);
        
        let x = self.gold.align_h(&crate::graphics::screen_rect(), HorizontalAlignment::Right);
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