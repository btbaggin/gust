
 use super::{EntityBehavior, EntityHandle, Entity, MAX_ENTITIES, EntityId};
 use crate::entity::GenerationalArray;
 use crate::messages::MessageBus;
 use std::collections::HashMap;
 use std::any::TypeId;

mod iter;
pub use iter::Iter;
crate::singleton!(entity_manager: EntityManager = EntityManager::new());

pub enum EntityCreationOptions {
    None,
    Tag,
    Persist,
}

struct EntityStorage {
    handle: EntityHandle,
    persist: bool,
}

pub struct EntityManager {
    entities: GenerationalArray<MAX_ENTITIES, Entity>,
    allocated: Vec<Option<EntityStorage>>,
    tags: HashMap<TypeId, EntityHandle>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager { 
            entities: GenerationalArray::new(),
            tags: HashMap::new(),
            allocated: Vec::with_capacity(MAX_ENTITIES / 2), // Assume half utilization of max entities
        }
    }

    pub fn create(&mut self, behavior: impl EntityBehavior + 'static) -> EntityHandle {
        self.create_options(behavior, EntityCreationOptions::None)
    }
    pub fn create_options(&mut self, behavior: impl EntityBehavior + 'static, options: EntityCreationOptions) -> EntityHandle {
        let id = behavior.id();
        let entity = Entity::new(behavior);
        let (handle, data) = self.entities.push(entity);
        data.initialize();

        let persist = matches!(options, EntityCreationOptions::Persist);
        let storage = EntityStorage { handle, persist, };
        self.allocated.insert(handle.index, Some(storage));

        //TODO make this use a vec
        match options {
            EntityCreationOptions::Tag | EntityCreationOptions::Persist => { self.tags.insert(id, handle); },
            _ => {},
        }
        handle
    }

    pub fn iter_handles(&self) -> Vec<EntityHandle> {
        self.allocated
            .iter()
            .filter(|e| e.is_some())
            .map(|e| e.as_ref().unwrap().handle)
            .collect()
    }

    pub fn iter(&self) -> iter::Iter {
        iter::Iter::new(self)
    }

    pub fn dispose_entities(&mut self, messages: &mut MessageBus) {
        for i in 0..self.allocated.len() {
            let handle = &self.allocated[i];
            if let Some(e) = handle {
                let entity = self.get(&e.handle).unwrap();
                if entity.mark_for_destroy {
                    if let Some(r) = entity.rigid_body {
                        crate::physics::RigidBody::destroy(r);
                    }
                    entity.behavior.unregister(messages);
                    self.entities.remove(&e.handle);
                    self.allocated[i] = None;
                }
            }
        }
        //TODO self.allocated = None;
    }
    pub fn get<'a>(&self, handle: &'a EntityHandle) -> Option<&Entity> {
        self.entities.get(handle)
    }
    pub fn get_mut<'a>(&mut self, handle: &'a EntityHandle) -> Option<&mut Entity> {
        self.entities.get_mut(handle)
    }
    pub fn find(&self, tag: EntityId) -> Option<&Entity> {
        if let Some(handle) = self.tags.get(&tag) {
            return self.entities.get(handle);
        }
        None
    }
    pub fn find_mut(&mut self, tag: EntityId) -> Option<&mut Entity> {
        if let Some(handle) = self.tags.get(&tag) {
            return self.entities.get_mut(handle);
        }
        None
    }
}

#[macro_export]
macro_rules! find_entity {
    ($( $manager:ident ).+, $ty:ty) => {{
        let mut typed_entity: Option<&$ty> = None;
        let address = std::any::TypeId::of::<$ty>();
        if let Some(entity) = $($manager.)+find(address) {
            typed_entity = entity.as_any().downcast_ref::<$ty>();
        }
        typed_entity
    }};
}
#[macro_export]
macro_rules! find_entity_mut {
    ($( $manager:ident ).+, $ty:ty) => {{
        let mut typed_entity: Option<&mut $ty> = None;
        let address = std::any::TypeId::of::<$ty>();
        if let Some(entity) = $($manager.)+find_mut(address) {
            typed_entity = entity.as_any_mut().downcast_mut::<$ty>();
        }
        typed_entity
    }};
}
#[macro_export]
macro_rules! get_entity {
    ($( $manager:ident ).+, $handle:ident, $ty:ty) => {{
        let mut typed_entity: Option<&$ty> = None;
        if let Some(entity) = $($manager.)+get(&$handle) {
            typed_entity = entity.as_any().downcast_ref::<$ty>();
        }
        typed_entity
    }};
}
#[macro_export]
macro_rules! get_entity_mut {
    ($( $manager:ident ).+, $handle:ident, $ty:ty) => {{
        let mut typed_entity: Option<&mut $ty> = None;
        if let Some(entity) = $($manager.)+get_mut(&$handle) {
            typed_entity = entity.as_any_mut().downcast_mut::<$ty>();
        }
        typed_entity
    }};
}