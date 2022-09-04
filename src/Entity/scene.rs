use crate::Graphics;
use super::{EntityManager, UpdateState};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{MessageBus, SharedMessageBus};
use crate::physics::QuadTree;
use speedy2d::shape::Rectangle;

pub enum SceneLoad {
    Load(Box<dyn SceneBehavior>),
    Unload,
    None,
}

pub struct Scene {
    behavior: Box<dyn SceneBehavior>,
    quad_tree: QuadTree,

}
impl Scene {
    pub fn new(behavior: Box<dyn SceneBehavior>, bounds: Rectangle) -> Scene {
        Scene { 
            behavior,
            quad_tree: QuadTree::new(bounds),
        }
    }
    
    pub fn load(&mut self, queue: ThreadSafeJobQueue, messages: SharedMessageBus, entities: &mut EntityManager) {
        let mut m = messages.borrow_mut();
        self.behavior.load(queue, &mut m, entities);
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

    pub fn update(&mut self, state: &mut crate::UpdateState) -> bool {
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
                    entity.update(state, &self.quad_tree);
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

    pub fn resize(&mut self, bounds: Rectangle) {
        self.quad_tree = QuadTree::new(bounds)
    }

    pub fn update_positions(&mut self, entities: &mut EntityManager) {
        //QuadTree will be one frame behind but that's fine because it's only used for course detection
        self.quad_tree.clear();
        for h in entities.iter_handles() {
            let entity = entities.get(&h).unwrap();
            let bounds = crate::math::sized_rect(entity.position, entity.scale);
            self.quad_tree.insert(h, bounds)
        }
    }
}

pub trait SceneBehavior: crate::messages::MessageHandler {
    fn load(&mut self, queue: ThreadSafeJobQueue, messages: &mut MessageBus, entities: &mut EntityManager);
    fn unload(&mut self);
    fn update(&mut self, update_state: &mut UpdateState) -> SceneLoad;
    fn render(&self, graphics: &mut Graphics);
}
