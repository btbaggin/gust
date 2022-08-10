use std::ops::{Deref, DerefMut};
use std::hash::Hash;
use std::collections::VecDeque;
use super::{EntityHandle, Entity, EntityTag, EntityManager};

pub struct Scene {
    handle: Box<dyn SceneHandle>,
    entities: Vec<EntityHandle>,
}
pub trait SceneHandle {
    fn load(&mut self, manager: &mut SceneManager);
    fn unload(&mut self, manager: &mut SceneManager);
}

pub struct SceneManager {
    scenes: VecDeque<Scene>,
    manager: EntityManager,
}
impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager { 
            scenes: VecDeque::new(), 
            manager: EntityManager::new(),
        }
    }

    pub fn current(&self) -> &Scene {
        self.scenes.back().expect("Scene manager must have a scene loaded")
    }
    fn current_mut(&mut self) -> &mut Scene {
        self.scenes.back_mut().expect("Scene manager must have a scene loaded")
    }

    pub fn add_handle_to_scene(&mut self, handle: EntityHandle) {
        //TODO we can't do this because the entity will be destroyed at the end
        //find someway to tag it so it persists
        self.current_mut().entities.push(handle);
    }

    pub fn add_to_scene(&mut self, entity: impl Entity + 'static) -> EntityHandle {
        let handle = self.manager.create(entity);
        self.current_mut().entities.push(handle);
        handle
    }

    pub fn add_tagged_to_scene(&mut self, entity: impl Entity + 'static, tag: EntityTag) -> EntityHandle {
        let handle = self.manager.create_tagged(entity, tag);
        self.current_mut().entities.push(handle);
        handle
    }

    pub fn load(&mut self, handle: Box<dyn SceneHandle>) {
        let mut scene = Scene { 
            handle,
            entities: vec!(),
        };
        scene.handle.load(self);
        self.scenes.push_back(scene);
    }

    pub fn unload(&mut self) {
        if let Some(mut scene) = self.scenes.pop_back() {
            scene.handle.unload(self);
            for e in scene.entities {
                self.manager.destroy(e);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &crate::input::Input) {
        for scene in &self.scenes {
            for e in &scene.entities {
                let entity = self.manager.get_mut(e).unwrap(); //TODO is this safe?
                entity.update(delta_time, input);
            }
        }
    }

    pub fn render(&mut self, graphics: &mut crate::Graphics) {
        for scene in &self.scenes {
            for e in &scene.entities {
                let entity = self.manager.get(e).unwrap(); //TODO is this safe?
                entity.render(graphics);
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