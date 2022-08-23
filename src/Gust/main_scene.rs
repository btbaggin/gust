use crate::entity::{SceneBehavior, EntityHandle, SceneManager, EntityManager, SceneLoad};
use crate::gust::{Player, Circle};
use crate::assets::{Sound, Sounds};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Message, MessageHandler, MessageBus};

pub struct MainLevel { }
impl MainLevel {
    pub fn new() -> MainLevel {
        MainLevel {}
    }
}
impl SceneBehavior for MainLevel {
    fn load(&mut self, manager: &mut SceneManager, queue: ThreadSafeJobQueue) -> Vec<EntityHandle> {
        //TODO some way to persist entities
        let mut results = vec!();
        let player = Player::new();
        results.push(manager.create_tagged(player));

        let circle = Circle::new();
        results.push(manager.create(circle));
        results
    }
    fn unload(&mut self) {}
    fn update(&mut self, _state: &mut crate::game_loop::UpdateState) -> SceneLoad {
        SceneLoad::None
    }
    fn render(&self, _graphics: &mut crate::Graphics) {}
}
impl MessageHandler for MainLevel {
    crate::set_address!(MainLevel);
    fn process(&mut self, message: &Message, message_bus: &mut MessageBus) {}
}