use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions, EntityManager, EntityHandle};
use crate::gust::{player::Player, enemy::EnemySpawner, tower::Tower, enemy::Wave, cards::{Manager, Card, Modifier, ModifierType}};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler, MessageContext};
use crate::assets::{Fonts};
use crate::ui::{LabeledValue, HorizontalAlignment, VerticalAlignment, DockContainer};
use crate::input::Actions;
use crate::graphics::Color;
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
    status: LevelStatus,
    indicator: Option<EntityHandle>,
    gold: u32,
}
impl Level {
    pub fn new() -> Level {
        Level { 
            spawner: EnemySpawner::new(),
            status: LevelStatus::InProgress,
            indicator: None,
            gold: 100
        }
    }

    fn try_purchase(&mut self, cost: u32, root: &mut crate::ui::Widget) -> bool {
        if cost > self.gold {
            return false
        }

        self.gold -= cost;
        if let Some(g) = root.find_mut::<LabeledValue<u32>>("gold") {
            g.set_value(self.gold);
        }
        true
    }

    fn handle_tower(&mut self, state: &mut crate::UpdateState, root: &mut crate::ui::Widget) {
        if state.action_pressed(Actions::SpawnTower1) && self.try_purchase(25, root) {
            let tower = Tower::new(10., 5., 200.);
            state.entities.create_at(tower, state.mouse_pos());
        }

        //TODO collision checking
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, entities: &mut EntityManager, root: &mut crate::ui::Widget) {    
        root.with_named_child(LabeledValue::new("Health", MAX_HEALTH, Fonts::Regular, 24., Color::WHITE), "health")
            .with_h_alignment(HorizontalAlignment::Left)
            .with_v_alignment(VerticalAlignment::Top);
        root.with_named_child(LabeledValue::new("Gold", 0u32, Fonts::Regular, 24., Color::WHITE), "gold")
            .with_h_alignment(HorizontalAlignment::Right)
            .with_v_alignment(VerticalAlignment::Top);

        let cards = vec![
            Card::new(vec![Modifier::new(ModifierType::Flat(5.), "attack_speed")], String::from("Increases attack speed")),
            Card::new(vec![Modifier::new(ModifierType::Percent(0.25), "damage")], String::from("Increases damage")),
            Card::new(vec![Modifier::new(ModifierType::Flat(100.), "range")], String::from("Increases range"))
        ];
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

        let player = Player::new();
        entities.create_options(player, EntityCreationOptions::Tag);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::UpdateState, root: &mut crate::ui::Widget) -> SceneLoad {
        self.spawner.update(state.delta_time, state.entities);
        self.handle_tower(state, root);

        //Show tower indicator
        if state.action_pressed(Actions::Select) {
            let entities = state.quad_tree.at_point::<Tower>(state.mouse_pos(), state.entities);
            assert!(entities.len() <= 1);

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
                self.indicator = Some(state.entities.create_at(crate::gust::tower::Indicator::new(t.range()), e.position));
            }
        }

        match self.status {
            LevelStatus::InProgress => SceneLoad::None,
            LevelStatus::Success => SceneLoad::Unload,
            LevelStatus::Fail => SceneLoad::Unload,
        }
    }
    fn render(&self, _graphics: &mut crate::Graphics) { }
}

impl MessageHandler for Level {
    crate::handle_messages!(Messages::EnemyGotToEnd(0), Messages::EnemyKilled);
    
    fn process(&mut self, message: &Messages, context: &mut MessageContext) { 
        match message {
            Messages::EnemyGotToEnd(damage) => {
                let h = context.root.find_mut::<LabeledValue<u32>>("health").unwrap();
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
                self.gold += 10;
                if let Some(g) = context.root.find_mut::<LabeledValue<u32>>("gold") {
                    g.set_value(self.gold);
                }
                if self.spawner.mark_enemy_dead() {
                    self.status = LevelStatus::Success
                }
            }
        }
    }
}
