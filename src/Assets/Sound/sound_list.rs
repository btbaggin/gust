use std::ops::{Deref, DerefMut};
use super::{PlayingSound, SoundHandle};
use std::collections::VecDeque;

crate::singleton!(sounds: SoundList = SoundList::new());

pub struct SoundList {
    sounds: Vec<Option<PlayingSound>>,
    free_indices: VecDeque<usize>
}
impl SoundList {
    pub fn new() -> SoundList {
        SoundList {
            sounds: Vec::with_capacity(64),
            free_indices: VecDeque::with_capacity(32),
        }
    }
    pub fn push(&mut self, sound: PlayingSound) -> SoundHandle {
        let inserted_at = if let Some(index) = self.free_indices.pop_front() {
            self.sounds[index] = Some(sound);
            index
        } else {
            self.sounds.push(Some(sound));
            self.sounds.len() - 1
        };
        inserted_at
    }

    pub fn get(&self, handle: SoundHandle) -> &Option<PlayingSound> {
        &self.sounds[handle]
    }
    
    pub fn get_mut(&mut self, handle: SoundHandle) -> &mut Option<PlayingSound> {
        &mut self.sounds[handle]
    }

    pub fn remove(&mut self, index: usize) {
        self.sounds[index] = None;
        self.free_indices.push_back(index);
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Option<PlayingSound>> {
        self.sounds.iter_mut()
    }
}