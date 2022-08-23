use std::ops::{Deref, DerefMut};
use std::collections::{VecDeque, HashSet};
use crate::entity::{EntityManager, EntityHandle};
use crate::entity::scene::{Scene, SceneBehavior, SceneLoad};
use crate::job_system::ThreadSafeJobQueue;
use crate::input::Input;
use crate::messages::{Message, MessageHandler, MessageBus, TypeAddress, MessageKind};
use std::cell::RefCell;
use std::rc::Rc;


pub struct SceneManager {
    scene: Option<Scene>,
    manager: EntityManager,
    queue: ThreadSafeJobQueue,
}
impl SceneManager {
    pub fn new(queue: ThreadSafeJobQueue) -> SceneManager {
        SceneManager { 
            scene: None, 
            manager: EntityManager::new(), //TODO move entity manager out to be global?
            queue: queue.clone(),
        }
    }

    pub fn load(&mut self, mut behavior: Box<dyn SceneBehavior>) {
        let entities = behavior.load(self, self.queue.clone());

        let scene = Scene { 
            behavior,
            entities,
        };
        self.scene = Some(scene);
    }

    pub fn unload(&mut self) {
        if let Some(scene) = &mut self.scene {
            scene.behavior.unload();

            for e in &scene.entities {
                self.manager.destroy(*e);
            }
            self.scene = None;
        }
    }

    pub fn update(&mut self, state: &mut crate::game_loop::UpdateState) -> bool {
        let mut new_scene = None;
        if let Some(scene) = &mut self.scene {
            let load = scene.behavior.update(state);
            
            match load {
                SceneLoad::Load(b) => { new_scene = Some(b); },
                SceneLoad::Unload => { },
                SceneLoad::None => { 
                    for e in &scene.entities {
                        let entity = self.manager.get_mut(e).unwrap();
                        entity.update(state);
                    }
                    self.process_messages(state.message_bus);
                    return true;
                 },
            }
        }
        
        self.unload();
        if let Some(b) = new_scene {
            self.load(b);
            return true;
        }
        return false;
    }

    pub fn process_messages(&mut self, messages: &mut crate::messages::MessageBus) {
        if let Some(scene) = &mut self.scene {
            while let Some(message) = messages.pop_message() {
                scene.behavior.process(&message, messages);

                for e in &scene.entities {
                    let entity = self.manager.get_mut(e).unwrap();
                    entity.behavior.process(&message, messages);
                }
            }
        }
    }

    pub fn render(&self, graphics: &mut crate::Graphics) {
        if let Some(scene) = &self.scene {
            for e in &scene.entities {
                let entity = self.manager.get(e).unwrap();
                entity.behavior.render(entity, graphics);
            }
            scene.behavior.render(graphics);
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