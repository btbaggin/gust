use std::collections::VecDeque;
use crate::utils::init_optional_array_to_blank;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct GenerationalIndex {
    pub index: usize,
    version: u16,
}

pub struct GenerationalArray<const C: usize, T: Sized> {
    data: [Option<T>; C],
    version: [u16; C],
    free: VecDeque<usize>,
    last_index: usize
}

impl<const C: usize, T: Sized> GenerationalArray<C, T> {
    pub fn new() -> GenerationalArray<C, T> {
        GenerationalArray {
            data: init_optional_array_to_blank::<T, C>(),
            version: [0; C],
            free: VecDeque::new(),
            last_index: 0
        }
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
}