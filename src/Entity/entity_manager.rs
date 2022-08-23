
 use super::{EntityBehavior, EntityHandle, Entity, MAX_ENTITIES};
 use crate::entity::GenerationalArray;
 use std::collections::HashMap;
 use std::any::TypeId;


crate::singleton!(entity_manager: EntityManager = EntityManager::new());

 pub struct EntityManager {
    entities: GenerationalArray<MAX_ENTITIES, Entity>,
    tags: HashMap<TypeId, EntityHandle>,
    old_entities: Vec<EntityHandle>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager { 
            entities: GenerationalArray::new(),
            tags: HashMap::new(),
            old_entities: vec!(),
        }
    }

    pub fn create(&mut self, behavior: impl EntityBehavior + 'static) -> EntityHandle {
        let entity = Entity::new(behavior);
        let (handle, data) = self.entities.push(entity);
        data.initialize();

        handle
    }
    pub fn create_tagged(&mut self, behavior: impl EntityBehavior + 'static) -> EntityHandle {
        let address = behavior.address();
        let entity = Entity::new(behavior);
        let (handle, data) = self.entities.push(entity);
        data.initialize();

        //TODO make this use a vec
        self.tags.insert(address, handle);
        handle
    }
    pub fn destroy(&mut self, handle: EntityHandle) {
        self.old_entities.push(handle);
    }
    pub fn dispose_entities(&mut self) {
        for e in &self.old_entities {
            if let Some(entity) = self.get(e) &&
               let Some(r) = entity.rigid_body {
                   crate::physics::RigidBody::destroy(r);
            }
            self.entities.remove(e);

        }
        self.old_entities.clear();
    }
    pub fn get<'a>(&self, handle: &'a EntityHandle) -> Option<&Entity> {
        self.entities.get(handle)
    }
    pub fn get_mut<'a>(&mut self, handle: &'a EntityHandle) -> Option<&mut Entity> {
        self.entities.get_mut(handle)
    }
    pub fn find<'a>(&self, tag: TypeId) -> Option<&Entity> {
        if let Some(handle) = self.tags.get(&tag) {
            return self.entities.get(handle);
        }
        None
    }
    pub fn find_mut<'a>(&mut self, tag: TypeId) -> Option<&mut Entity> {
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
        if let Some(entity) = $($manager.)+find(&address) {
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
        if let Some(entity) = $($manager.)+find_mut(&address) {
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