use crate::entity::{SceneBehavior, SceneLoad, EntityCreationOptions};
use crate::gust::{player::Player, enemy::EnemySpawner, enemy::Wave, enemy::EnemyType};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Message, MessageHandler, MessageBus, SharedMessageBus, MessageKind};
use crate::V2U;

pub struct Level { 
    spawner: EnemySpawner,
    layout: Vec<V2U>,
}
impl Level {
    pub fn new(messages: SharedMessageBus) -> Level {
        //TODO tile size based on screen size
        Level { 
            spawner: EnemySpawner::new(messages),
            layout: vec!(V2U::new(300, 300)),
        }
    }
}
impl SceneBehavior for Level {
    fn load(&mut self, _queue: ThreadSafeJobQueue, messages: SharedMessageBus) {
        let entity_manager = crate::entity::entity_manager();
        let player = Player::new();
        entity_manager.create_options(player, EntityCreationOptions::Tag);

        self.spawner.add_wave(Wave::new(5, 0.5, EnemyType::Slime), 2.);
        self.spawner.add_wave(Wave::new(10, 1., EnemyType::Slime), 5.);
    }
    fn unload(&mut self) {}
    fn update(&mut self, state: &mut crate::game_loop::UpdateState) -> SceneLoad {
        self.spawner.update(state.delta_time);
        SceneLoad::None
    }
    fn render(&self, _graphics: &mut crate::Graphics) {}
}
impl MessageHandler for Level {
    crate::set_address!(Level);
    fn process(&mut self, message: &Message, _message_bus: &mut MessageBus) {
        match message.kind() {
            MessageKind::SpawnEnemy => {
                let manager = crate::entity::entity_manager();
                let position = crate::V2::new(self.layout[0].x as f32, self.layout[0].y as f32);
                manager.create(crate::gust::enemy::Slime::new(position));
            }
        }
    }
}