
 use super::{EntityBehavior, EntityHandle, Entity, MAX_ENTITIES, EntityId};
 use crate::generational_array::{GenerationalArray, Iter};
 use crate::messages::MessageBus;
 use crate::V2;
 use std::collections::HashMap;
 use std::any::TypeId;

// mod iter;
// pub use iter::Iter;
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

        match options {
            EntityCreationOptions::Tag => { self.tags.insert(id, handle); },
            _ => {},
        }
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
                if let Some(r) = entity.rigid_body {
                    crate::physics::RigidBody::destroy(r);
                }
                entity.behavior.unregister(messages);
                self.entities.remove(&h);
            }
        }
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