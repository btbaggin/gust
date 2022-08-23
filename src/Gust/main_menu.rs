use crate::entity::{SceneBehavior, EntityHandle, SceneManager, SceneLoad};
use crate::gust::{Player, Circle};
use crate::assets::{Sound, Sounds, SoundHandle, SoundStatus, Fonts};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Message, MessageHandler, MessageBus};
use crate::{V2, Graphics, Label};
use crate::input::Actions;

pub struct MainMenu {
    selected_index: usize,
    audio_handle: Option<SoundHandle>,
    labels: Vec<Label>,
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu { 
            selected_index: 0,
            audio_handle: None,
            labels: vec!(),
        }
    }
}
impl SceneBehavior for MainMenu {
    fn load(&mut self, manager: &mut SceneManager, queue: ThreadSafeJobQueue) -> Vec<EntityHandle> {
        self.audio_handle = Some(Sound::play(&queue, Sounds::Piano));
        self.labels.push(Label::new(String::from("New Game"), Fonts::Regular, 64.));
        self.labels.push(Label::new(String::from("Settings"), Fonts::Regular, 64.));
        self.labels.push(Label::new(String::from("Exit"), Fonts::Regular, 64.));
        vec!() //TODO none?
    }
    fn unload(&mut self) {
        let sound = Sound::get_mut(self.audio_handle.unwrap()).unwrap();
        sound.set_status(SoundStatus::Stopped);
    }
    fn update(&mut self, state: &mut crate::game_loop::UpdateState) -> SceneLoad {
        if state.input.action_pressed(&Actions::Up) && self.selected_index > 0 { 
            self.selected_index -= 1;
        }
        if state.input.action_pressed(&Actions::Down) && self.selected_index < self.labels.len() - 1 { 
            self.selected_index += 1;
        }

        if state.input.action_pressed(&Actions::Accept) {
            match self.selected_index {
                0 => return SceneLoad::Load(Box::new(crate::gust::MainLevel::new())),
                1 => return SceneLoad::None,
                2 => return SceneLoad::Unload,
                _ => panic!("Invalid selection index"),
            }
        }
        SceneLoad::None
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        let mut pos = crate::V2::new(0., 0.);

        for (i, l) in self.labels.iter().enumerate() {
            let color = if self.selected_index == i { speedy2d::color::Color::RED } else { speedy2d::color::Color::WHITE };
            l.render(graphics, pos, color);
            pos.y += 100.;
        }
        
    }
}
impl MessageHandler for MainMenu {
    crate::set_address!(MainMenu);
    fn process(&mut self, message: &Message, message_bus: &mut MessageBus) {}
}