use crate::entity::{SceneBehavior, EntityTag, EntityHandle, EntityManager};
use crate::gust::{Player, Circle};
use crate::assets::{Sound, Sounds};
use crate::job_system::ThreadSafeJobQueue;

pub struct MainLevel { }
impl MainLevel {
    pub fn new() -> MainLevel {
        MainLevel {}
    }
}
impl SceneBehavior for MainLevel {
    fn load(&mut self, manager: &mut EntityManager, queue: ThreadSafeJobQueue) -> Vec<EntityHandle> {
        //TODO some way to persist entities
        let mut results = vec!();
        let player = Player::new();
        results.push(manager.create_tagged(player, EntityTag::Player));

        let circle = Circle::new();
        results.push(manager.create(circle));

        Sound::play(&queue, Sounds::Piano);
        results
    }
    fn unload(&mut self, manager: &mut EntityManager) {}
    fn update(&mut self, delta_time: f32) {}
    fn render(&self, graphics: &mut crate::Graphics) {}
}