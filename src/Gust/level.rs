use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions};
use crate::gust::{player::Player, enemy::EnemySpawner, enemy::Wave, enemy::EnemyType};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Message, MessageHandler, MessageBus, SharedMessageBus, MessageKind};
use crate::assets::{Fonts};
use crate::ui::Label;
use crate::{V2U, V2};

const MAX_HEALTH: u32 = 100;

pub struct Level { 
    spawner: EnemySpawner,
    health: u32,
    health_label: Label
}
impl Level {
    pub fn new() -> Level {
        Level { 
            spawner: EnemySpawner::new(),
            health: MAX_HEALTH,
            health_label: Label::new(MAX_HEALTH.to_string(), Fonts::Regular, 24.),
        }
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, _messages: &mut MessageBus) {
        let entity_manager = crate::entity::entity_manager();
        let player = Player::new();
        entity_manager.create_options(player, EntityCreationOptions::Tag);

        self.spawner.add_wave(Wave::new(5, 0.5, EnemyType::Slime), 2.);
        self.spawner.add_wave(Wave::new(10, 1., EnemyType::Slime), 5.);

        let positions = vec!(V2U::new(0, 10), V2U::new(10, 10), V2U::new(10, 3), V2U::new(20, 3));
        let layout = crate::gust::level_layout::LevelLayout::new(positions);
        entity_manager.create_options(layout, EntityCreationOptions::Tag);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::UpdateState) -> SceneLoad {
        self.spawner.update(state.delta_time);

        if self.health == 0 { SceneLoad::Unload }
        else { SceneLoad::None }
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        self.health_label.render(graphics, V2::new(0., 0.), speedy2d::color::Color::WHITE)
    }
}
impl MessageHandler for Level {
    crate::handle_messages!(MessageKind::EnemyGotToEnd);
    fn process(&mut self, message: &Message) { 
        match message.kind() {
            MessageKind::EnemyGotToEnd => {
                self.health -= 10;
                self.health_label = Label::new(self.health.to_string(), Fonts::Regular, 24.);
            }
        }
    }
}