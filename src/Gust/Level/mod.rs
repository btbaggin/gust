use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions, EntityManager, EntityHandle};
use crate::gust::{player::Player, enemy::EnemySpawner, enemy::Wave, cards::Manager, cards::Card};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler};
use crate::assets::{Fonts};
use crate::ui::{LabeledValue, HorizontalAlignment, VerticalAlignment, DockContainer};
use crate::V2;
use crate::input::Actions;
use crate::graphics::Color;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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
    status: LevelStatus,
    indicator: Option<EntityHandle>,
}
impl Level {
    pub fn new() -> Level {
        Level { 
            spawner: EnemySpawner::new(),
            status: LevelStatus::InProgress,
            indicator: None
        }
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, entities: &mut EntityManager) {    
        let root = crate::ui::root();
        root.with_named_child(LabeledValue::new("Health", MAX_HEALTH, Fonts::Regular, 24., Color::WHITE), "health")
            .with_h_alignment(HorizontalAlignment::Left)
            .with_v_alignment(VerticalAlignment::Top);
        root.with_named_child(LabeledValue::new("Gold", 0u32, Fonts::Regular, 24., Color::WHITE), "gold")
            .with_h_alignment(HorizontalAlignment::Right)
            .with_v_alignment(VerticalAlignment::Top);

        let cards = vec![Card::new(vec!()), Card::new(vec!()), Card::new(vec!())];
        root.with_named_child(Manager::new(cards), "card_manager")
            .with_v_alignment(VerticalAlignment::Bottom);

        let file = std::fs::read_to_string("./resources/levels.ron").unwrap();
        let v = ron::from_str::<HashMap<&str, crate::gust::game_data::Level>>(&file).unwrap();
        let level = &v["level_1"];

        for w in &level.waves {
            self.spawner.add_wave(Wave::new(w.count, w.interval, w.r#type), w.time);
        }

        let layout = crate::gust::level::Layout::new(&level.layout);
        entities.create_options(layout, EntityCreationOptions::Tag);

        for t in &level.towers {
            let tower = crate::gust::tower::Tower::new(10., 5., 200.);
            entities.create_at(tower, V2::new(t.0 as f32 * 32., t.1 as f32 * 32.));
        }

        let player = Player::new();
        entities.create_options(player, EntityCreationOptions::Tag);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::UpdateState) -> SceneLoad {
        self.spawner.update(state.delta_time, state.entities);

        //Show tower indicator
        if state.action_pressed(Actions::Select) {
            let entities = state.quad_tree.at_point::<crate::gust::tower::Tower>(state.mouse_pos(), state.entities);
            assert!(entities.len() <= 1);

            let root = crate::ui::root();
            if let Some(i) = self.indicator && 
               let Some(e) = state.entities.get_mut(&i) {
                e.destroy();
                root.remove("stats");
            }

            if entities.len() == 1 {
                let e = state.entities.get(&entities[0]).unwrap();
                let t = e.as_any().downcast_ref::<crate::gust::tower::Tower>().unwrap();
                
                root.with_named_child(DockContainer::vertical(0.25, 0.25, Some(Color::GRAY)), "stats")
                    .with_h_alignment(HorizontalAlignment::Right)
                    .with_v_alignment(VerticalAlignment::Bottom)
                    .add_child(LabeledValue::new("Damage", t.damage(), Fonts::Regular, 24., Color::WHITE))
                    .add_child(LabeledValue::new("Attack Rate", t.attack_speed(), Fonts::Regular, 24., Color::WHITE))
                    .add_child(LabeledValue::new("Range", t.range(), Fonts::Regular, 24., Color::WHITE));
                self.indicator = Some(state.entities.create_at(crate::gust::tower::Indicator::new(200.), e.position));
            }
        }

        match self.status {
            LevelStatus::InProgress => SceneLoad::None,
            LevelStatus::Success => SceneLoad::Unload,
            LevelStatus::Fail => SceneLoad::Unload,
        }
    }
    fn render(&self, graphics: &mut crate::Graphics) { }
}

impl MessageHandler for Level {
    crate::handle_messages!(Messages::EnemyGotToEnd(0), Messages::EnemyKilled);
    
    fn process(&mut self, message: &Messages) { 
        match message {
            Messages::EnemyGotToEnd(damage) => {
                let root = crate::ui::root();
                let h = root.find_mut::<LabeledValue<u32>>("health").unwrap();
                let current = h.value();
                let damage = u32::min(*damage, current);
                h.set_value(current - damage);

                // This code needs to run unconditionally
                let level_done = self.spawner.mark_enemy_dead();
                if h.value() == 0 {
                    self.status = LevelStatus::Fail
                } else if level_done {
                    self.status = LevelStatus::Success
                }
            },
            Messages::EnemyKilled => {
                let root = crate::ui::root();
                let g = root.find_mut::<LabeledValue<u32>>("gold").unwrap();
                let gold = g.value();
                g.set_value(gold + 10);
                if self.spawner.mark_enemy_dead() {
                    self.status = LevelStatus::Success
                }
            }
        }
    }
}