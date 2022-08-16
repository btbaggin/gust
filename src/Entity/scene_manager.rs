use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;
use crate::entity::{EntityManager, EntityHandle};
use crate::entity::scene::{Scene, SceneBehavior};

pub struct SceneManager {
    scenes: VecDeque<Scene>,
    manager: EntityManager,
    entities: Vec<EntityHandle>,
}
impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager { 
            scenes: VecDeque::new(), 
            manager: EntityManager::new(),
            entities: vec!(),
        }
    }

    pub fn current(&self) -> &Scene {
        self.scenes.back().expect("Scene manager must have a scene loaded")
    }
    fn current_mut(&mut self) -> &mut Scene {
        self.scenes.back_mut().expect("Scene manager must have a scene loaded")
    }

    pub fn load(&mut self, mut handle: Box<dyn SceneBehavior>, queue: crate::job_system::ThreadSafeJobQueue) {
        let entities = handle.load(&mut self.manager, queue);

        let mut scene = Scene { 
            handle,
            entities,
        };
        self.scenes.push_back(scene);
    }

    pub fn unload(&mut self) {
        if let Some(mut scene) = self.scenes.pop_back() {
            scene.handle.unload(&mut self.manager);
            for e in scene.entities {
                self.manager.destroy(e);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &crate::input::Input) {
        for e in &self.entities {
            let entity = self.manager.get_mut(e).unwrap(); //TODO is this safe?
            let p = entity as *mut crate::entity::Entity as u64;
            entity.update(delta_time, input);
        }

        for scene in &self.scenes {
            for e in &scene.entities {
                let entity = self.manager.get_mut(e).unwrap(); //TODO is this safe?
                entity.update(delta_time, input);
            }
        }
    }

    pub fn render(&self, graphics: &mut crate::Graphics) {
        for e in &self.entities {
            let entity = self.manager.get(e).unwrap(); //TODO is this safe?
            entity.behavior.render(entity, graphics);
        }

        for scene in &self.scenes {
            for e in &scene.entities {
                let entity = self.manager.get(e).unwrap(); //TODO is this safe?
                entity.behavior.render(entity, graphics);
            }
        }
    }
}
impl Deref for SceneManager {
    type Target = EntityManager;
    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}
impl DerefMut for SceneManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.manager
    }
}