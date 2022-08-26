use crate::Graphics;
use super::{EntityManager, UpdateState};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{MessageBus, SharedMessageBus};


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
        let mut m = messages.borrow_mut();
        self.behavior.load(queue, &mut m);
        self.behavior.register(&mut m)
    }

    fn unload(&mut self, entities: &mut EntityManager, messages: &SharedMessageBus) {
        let mut m = messages.borrow_mut();
        self.behavior.unload();
        self.behavior.unregister(&mut m);
    
        for h in entities.iter_handles() {
            let entity = entities.get_mut(&h).unwrap();
            entity.destroy();
        }
    }

    pub fn update(&mut self, state: &mut crate::game_loop::UpdateState, entities: &mut EntityManager) -> bool {
        let load = self.behavior.update(state);
        
        match load {
            SceneLoad::Load(b) => { 
                self.unload(entities, &state.message_bus);
                self.behavior = b;
                self.load(state.queue.clone(), state.message_bus.clone());
                true
             },
            SceneLoad::Unload => {
                self.unload(entities, &state.message_bus);
                false
            },
            SceneLoad::None => { 
                for h in entities.iter_handles() {
                    let entity = entities.get_mut(&h).unwrap();
                    entity.update(state);
                }
                let mut m = state.message_bus.borrow_mut();
                m.process_messages();
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
}

pub trait SceneBehavior: crate::messages::MessageHandler {
    fn load(&mut self, queue: ThreadSafeJobQueue, messages: &mut MessageBus);
    fn unload(&mut self);
    fn update(&mut self, update_state: &mut UpdateState) -> SceneLoad;
    fn render(&self, graphics: &mut Graphics);
}
