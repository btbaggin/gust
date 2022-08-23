use crate::Graphics;
use super::{EntityHandle, EntityManager, SceneManager, UpdateState};
use crate::job_system::ThreadSafeJobQueue;


pub enum SceneLoad {
    Load(Box<dyn SceneBehavior>),
    Unload,
    None,
}

pub struct Scene {
    pub(super) behavior: Box<dyn SceneBehavior>,
    pub(super) entities: Vec<EntityHandle>,
}
pub trait SceneBehavior: crate::messages::MessageHandler {
    fn load(&mut self, manager: &mut SceneManager, queue: ThreadSafeJobQueue) -> Vec<EntityHandle>;
    fn unload(&mut self);
    fn update(&mut self, update_state: &mut UpdateState) -> SceneLoad;
    fn render(&self, graphics: &mut Graphics);
}
