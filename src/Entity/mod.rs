use std::collections::HashMap;
use std::hash::Hash;

mod generational_array;
pub use generational_array::{GenerationalArray, GenerationalIndex};

const MAX_ENTITIES: usize = 512;

pub type EntityHandle = GenerationalIndex;
pub struct EntityManager<T: Hash + Eq> {
    entities: GenerationalArray<MAX_ENTITIES, Box<dyn Entity>>,
    tags: HashMap<T, EntityHandle>,
    old_entities: Vec<EntityHandle>,
}

pub trait Entity {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[macro_export]
macro_rules! create_entity {
    (pub struct $name:ident {
        $($field:ident: $ty:ty,)+
    }) => {
        pub struct $name {
            $($field: $ty,)+
        }
        impl crate::entity::Entity for $name {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
        }
    };
}

impl<T: Hash + Eq> EntityManager<T> {
    pub fn new() -> EntityManager<T> {
        EntityManager { 
            entities: GenerationalArray::new(),
            tags: HashMap::new(),
            old_entities: vec!(),
        }
    }
    pub fn create(&mut self, entity: impl Entity + 'static) -> EntityHandle {
        self.entities.push(Box::new(entity))
    }
    pub fn create_tagged(&mut self, entity: impl Entity + 'static, tag: T) -> EntityHandle {
        let handle = self.entities.push(Box::new(entity));
        self.tags.insert(tag, handle.clone());
        handle
    }
    pub fn destroy(&mut self, handle: EntityHandle) {
        self.old_entities.push(handle);
    }
    pub(super) fn clear_entities(&mut self) {
        for e in &self.old_entities {
            self.entities.remove(e);
        }
        self.old_entities.clear();
    }
    pub fn get<'a>(&self, handle: &'a EntityHandle) -> Option<&Box<dyn Entity>> {
        self.entities.get(handle)
    }
    pub fn get_mut<'a>(&mut self, handle: &'a EntityHandle) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(handle)
    }
    pub fn find<'a>(&self, tag: &'a T) -> Option<&Box<dyn Entity>> {
        if let Some(handle) = self.tags.get(tag) {
            return self.entities.get(handle);
        }
        None
    }
    pub fn find_mut<'a>(&mut self, tag: &'a T) -> Option<&mut Box<dyn Entity>> {
        if let Some(handle) = self.tags.get(tag) {
            return self.entities.get_mut(handle);
        }
        None
    }
}

#[macro_export]
macro_rules! find_entity {
    ($( $manager:ident ).+, $tag:path, $ty:ty) => {{
        let mut typed_entity: Option<&$ty> = None;
        if let Some(entity) = $($manager.)+find(&$tag) {
            typed_entity = entity.as_any().downcast_ref::<$ty>();
        }
        typed_entity
    }};
}
#[macro_export]
macro_rules! find_entity_mut {
    ($( $manager:ident ).+, $tag:path, $ty:ty) => {{
        let mut typed_entity: Option<&mut $ty> = None;
        if let Some(entity) = $($manager.)+find_mut(&$tag) {
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