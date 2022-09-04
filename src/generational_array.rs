use std::collections::VecDeque;
use crate::utils::init_optional_array_to_blank;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct GenerationalIndex {
    pub index: usize,
    version: u16,
}

pub struct GenerationalArray<T: Sized, const C: usize> {
    data: [Option<T>; C],
    version: [u16; C],
    allocated: [u16; C],
    free: VecDeque<usize>,
    last_index: usize
}

impl<T: Sized, const C: usize> GenerationalArray<T, C> {
    pub fn new() -> GenerationalArray<T, C> {
        GenerationalArray {
            data: init_optional_array_to_blank::<T, C>(),
            version: [0; C],
            allocated: [0; C],
            free: VecDeque::new(),
            last_index: 0
        }
    }

    pub fn len(&self) -> usize {
        self.last_index
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        self.data[index].as_ref()
    }
    pub fn get_at_mut(&mut self, index: usize) -> Option<&mut T> {
        unsafe { self.data.get_unchecked_mut(index).as_mut() }
    }

    pub fn get(&self, index: &GenerationalIndex) -> Option<&T> {
        let version = self.version[index.index];
        if version == index.version {
            return self.data[index.index].as_ref()
        }
        None
    }

    pub fn get_mut(&mut self, index: &GenerationalIndex) -> Option<&mut T> {
        let version = self.version[index.index];
        if version == index.version {
            return self.data[index.index].as_mut()
        }
        None
    }

    pub fn push(&mut self, value: T) -> (GenerationalIndex, &mut T) {
        let index = match self.free.pop_front() {
            Some(i) => i,
            None => {
                self.last_index += 1;
                self.last_index - 1
            }
        };

        self.data[index] = Some(value);
        self.allocated[index] = self.version[index];
        
        let data = self.data[index].as_mut().unwrap();
        let index = GenerationalIndex { index, version: self.version[index] };
        (index, data)
    }

    pub fn remove(&mut self, index: &GenerationalIndex) -> bool {
        if index.version == self.version[index.index] {

            self.data[index.index] = None;
            self.version[index.index] += 1;

            self.free.push_back(index.index);
            return true;
        }

        false
    }

    pub fn iter(&self) -> Iter<T, C> {
        Iter::new(self)
    }
    pub fn iter_index(&self) -> Vec<GenerationalIndex> {
        let mut ret = vec!();
        for i in 0..self.last_index {
            if self.version[i] == self.allocated[i] {
                ret.push(GenerationalIndex { index: i, version: self.version[i] });
            }
        }
        ret
    }
}


pub struct Iter<'a, T: Sized, const C: usize> {
    array: &'a GenerationalArray<T, C>,
    current: usize,
}
impl<'a, T: Sized, const C: usize> Iter<'a, T, C> {
    pub fn new(array: &'a GenerationalArray<T, C>) -> Iter<'a, T, C> {
        Iter {
            array,
            current: 0
        }
    }
}
impl<'a, T: Sized, const C: usize> Iterator for Iter<'a, T, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current >= self.array.last_index {
            return None;
        }
        while self.array.version[self.current] != self.array.allocated[self.current] {
            self.current += 1;
            if self.current >= self.array.last_index {
                return None;
            }
        }
        let data = self.array.data[self.current].as_ref();
        self.current += 1;
        data
    }
}