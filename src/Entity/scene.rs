use crate::Graphics;
use super::{EntityManager, UpdateState};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{SharedMessageBus};

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
    
    pub fn load(&mut self, queue: ThreadSafeJobQueue, messages: SharedMessageBus, entities: &mut EntityManager) {
        let mut m = messages.borrow_mut();
        self.behavior.load(queue, entities);
        self.behavior.register(&mut m)
    }

    fn unload(&mut self, entities: &mut EntityManager, messages: &SharedMessageBus) {
        let mut m = messages.borrow_mut();
        self.behavior.unload();
        self.behavior.unregister(&mut m);
    
        // Unload entities
        for h in entities.iter_handles() {
            let entity = entities.get_mut(&h).unwrap();
            entity.destroy();
        }

        // Unload UI
        let root = crate::ui::root();
        for c in root.children_mut() {
            c.destroy();
        }
    }

    pub fn update(&mut self, state: &mut crate::UpdateState) -> bool {
        let root = crate::ui::root();
        root.layout(&crate::graphics::screen_rect());
        root.update(state);

        let load = self.behavior.update(state);
        
        match load {
            SceneLoad::Load(b) => { 
                self.unload(state.entities, &state.message_bus);
                self.behavior = b;
                self.load(state.queue.clone(), state.message_bus.clone(), state.entities);
                true
             },
            SceneLoad::Unload => {
                self.unload(state.entities, &state.message_bus);
                false
            },
            SceneLoad::None => { 
                let entities = crate::entity::entity_manager();
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

        let root = crate::ui::root();
        root.render(graphics, &crate::graphics::screen_rect())
    }
}

pub trait SceneBehavior: crate::messages::MessageHandler {
    fn load(&mut self, queue: ThreadSafeJobQueue, entities: &mut EntityManager);
    fn unload(&mut self);
    fn update(&mut self, update_state: &mut UpdateState) -> SceneLoad;
    fn render(&self, graphics: &mut Graphics);
}
