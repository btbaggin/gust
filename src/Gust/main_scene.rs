use crate::entity::{SceneHandle, SceneManager};

pub struct MainLevel { }
impl MainLevel {
    pub fn new() -> MainLevel {
        MainLevel {}
    }
}
impl SceneHandle for MainLevel {
    fn load(&mut self, manager: &mut SceneManager) {

    }
    fn unload(&mut self, manager: &mut SceneManager) {

    }
}