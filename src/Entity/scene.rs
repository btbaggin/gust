use crate::Graphics;
use super::{EntityManager, UpdateState};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::SharedMessageBus;


pub enum SceneLoad {
    Load(Box<dyn SceneBehavior>),
    Unload,
    None,
}

pub struct Scene {
    behavior: Box<dyn SceneBehavior>,
}
impl Scene {
    pub fn new(behavior: Box<dyn SceneBehavior>) -> Scene {
        Scene { behavior }
    }
    
    pub fn load(&mut self, queue: ThreadSafeJobQueue, messages: SharedMessageBus) {
        self.behavior.load(queue, messages)
    }

    fn unload(&mut self, entities: &mut EntityManager) {
        self.behavior.unload();
    
        for h in entities.iter_handles() {
            let entity = entities.get_mut(&h).unwrap();
            entity.destroy();
        }
    }

    pub fn update(&mut self, state: &mut crate::game_loop::UpdateState, entities: &mut EntityManager) -> bool {
        let load = self.behavior.update(state);
        
        match load {
            SceneLoad::Load(b) => { 
                self.unload(entities);
                self.behavior = b;
                self.load(state.queue.clone(), state.message_bus.clone());
                true
             },
            SceneLoad::Unload => {
                self.unload(entities);
                false
            },
            SceneLoad::None => { 
                for h in entities.iter_handles() {
                    let entity = entities.get_mut(&h).unwrap();
                    entity.update(state);
                }
                self.process_messages(&state.message_bus, entities);
                true
            },
        }
    }

    pub fn render(&self, graphics: &mut crate::Graphics, entities: &EntityManager) {
        for entity in entities.iter() {
            entity.behavior.render(entity, graphics);
        }
        self.behavior.render(graphics);
    }

    pub fn process_messages(&mut self, messages: &SharedMessageBus, manager: &mut EntityManager) {
        let mut messages = messages.borrow_mut();
        while let Some(message) = messages.pop_message() {
            self.behavior.process(&message, &mut messages);

            for h in manager.iter_handles() {
                let entity = manager.get_mut(&h).unwrap();
                entity.behavior.process(&message, &mut messages);
            }
        }
    }
}

pub trait SceneBehavior: crate::messages::MessageHandler {
    fn load(&mut self, queue: ThreadSafeJobQueue, messages: SharedMessageBus);
    fn unload(&mut self);
    fn update(&mut self, update_state: &mut UpdateState) -> SceneLoad;
    fn render(&self, graphics: &mut Graphics);
}
