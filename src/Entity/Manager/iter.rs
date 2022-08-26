use super::{EntityManager, Entity, MAX_ENTITIES, EntityStorage, GenerationalArray};

pub struct Iter<'a> {
    allocated: &'a Vec<Option<EntityStorage>>,
    entities: &'a GenerationalArray<MAX_ENTITIES, Entity>,
    current: usize,
}
impl<'a> Iter<'a> {
    pub fn new(manager: &'a EntityManager) -> Iter<'a> {
        Iter {
            allocated: &manager.allocated,
            entities: &manager.entities,
            current: 0
        }
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<&'a Entity> {
        if self.current >= self.allocated.len() {
            return None;
        }
        while self.allocated[self.current].is_none() {
            self.current += 1;
        }
        let handle = self.allocated[self.current].as_ref().unwrap().handle;
        self.current += 1;
        Some(self.entities.get(&handle).unwrap())
    }
}
