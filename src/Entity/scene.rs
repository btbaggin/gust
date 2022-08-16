use crate::Graphics;
use super::{EntityHandle, EntityManager, SceneManager};

pub struct Scene {
    pub(super) handle: Box<dyn SceneBehavior>,
    pub(super) entities: Vec<EntityHandle>,
}
pub trait SceneBehavior {
    fn load(&mut self, manager: &mut EntityManager, queue: crate::job_system::ThreadSafeJobQueue) -> Vec<EntityHandle>;
    fn unload(&mut self, manager: &mut EntityManager);
    fn update(&mut self, delta_time: f32);
    fn render(&self, graphics: &mut Graphics);
}
