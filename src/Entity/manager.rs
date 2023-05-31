
 use super::{EntityBehavior, EntityHandle, Entity, MAX_ENTITIES};
 use crate::generational_array::{GenerationalArray, Iter};
 use crate::messages::MessageBus;
 use crate::V2;
 use std::collections::HashMap;
 use std::any::TypeId;

crate::singleton!(entity_manager: EntityManager = EntityManager::new());

pub enum EntityCreationOptions {
    None,
    Tag,
    //TODO persist
}

pub struct EntityManager {
    entities: GenerationalArray<Entity, MAX_ENTITIES>,
    tags: HashMap<TypeId, EntityHandle>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager { 
            entities: GenerationalArray::new(),
            tags: HashMap::new(),
        }
    }

    pub fn create(&mut self, behavior: impl EntityBehavior + 'static) -> EntityHandle {
        self.create_options_at(behavior, V2::new(0., 0.), EntityCreationOptions::None)
    }
    pub fn create_at(&mut self, behavior: impl EntityBehavior + 'static, position: V2) -> EntityHandle {
        self.create_options_at(behavior, position, EntityCreationOptions::None)
    }
    pub fn create_options(&mut self, behavior: impl EntityBehavior + 'static, options: EntityCreationOptions) -> EntityHandle {
        self.create_options_at(behavior, V2::new(0., 0.), options)
    }
    pub fn create_options_at(&mut self, behavior: impl EntityBehavior + 'static, position: V2, options: EntityCreationOptions) -> EntityHandle {
        let id = behavior.id();
        let entity = Entity::new(behavior, position);
        let (handle, data) = self.entities.push(entity);
        data.initialize();

        if let EntityCreationOptions::Tag = options { self.tags.insert(id, handle); }

        handle
    }

    pub fn iter_handles(&self) -> Vec<EntityHandle> {
        self.entities.iter_index()
    }
    pub fn iter(&self) -> Iter<Entity, MAX_ENTITIES> {
        self.entities.iter()
    }

    pub fn dispose_entities(&mut self, messages: &mut MessageBus) {
        for h in self.iter_handles() {
            let entity = self.get(&h).unwrap();
            if entity.mark_for_destroy {
                let id = entity.behavior.id();
                if let Some(r) = entity.rigid_body {
                    crate::physics::RigidBody::destroy(r);
                }

                entity.behavior.unregister(messages);
                self.entities.remove(&h);
                self.tags.remove(&id);
                
            }
        }
    }
    pub fn get(&self, handle: &EntityHandle) -> Option<&Entity> {
        self.entities.get(handle)
    }
    pub fn get_mut(&mut self, handle: &EntityHandle) -> Option<&mut Entity> {
        self.entities.get_mut(handle)
    }

    pub fn find_as<T: 'static>(&self) -> Option<&T> {
        let address = std::any::TypeId::of::<T>();

        if let Some(handle) = self.tags.get(&address) {
            let entity = self.entities.get(handle).unwrap();
            return entity.as_any().downcast_ref::<T>();
        }
        None
    }

    pub fn find_as_mut<T: 'static>(&mut self) -> Option<&T> {
        let address = std::any::TypeId::of::<T>();
        
        if let Some(handle) = self.tags.get(&address) {
            let entity = self.entities.get_mut(handle).unwrap();
            return entity.as_any().downcast_ref::<T>();
        }
        None
    }
}
impl Default for EntityManager {
    fn default() -> Self { Self::new() }
}