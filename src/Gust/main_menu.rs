use crate::entity::{SceneBehavior, SceneLoad, EntityManager};
use crate::assets::{Sound, Sounds, SoundHandle, SoundStatus, Fonts};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler, MessageBus};
use crate::ui::{Label, UiElement, HorizontalAlignment};
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
    fn load(&mut self, queue: ThreadSafeJobQueue, _messages: &mut MessageBus, _entities: &mut EntityManager) {
        self.audio_handle = Some(Sound::play(&queue, Sounds::Piano));
        self.labels.push(Label::new(String::from("New Game"), Fonts::Regular, 64.));
        self.labels.push(Label::new(String::from("Settings"), Fonts::Regular, 64.));
        self.labels.push(Label::new(String::from("Exit"), Fonts::Regular, 64.));
    }
    fn unload(&mut self) {
        let sound = Sound::get_mut(self.audio_handle.unwrap()).unwrap();
        sound.set_status(SoundStatus::Stopped);
    }
    fn update(&mut self, state: &mut crate::UpdateState) -> SceneLoad {
        if state.action_pressed(&Actions::Up) && self.selected_index > 0 { 
            self.selected_index -= 1;
        }
        if state.action_pressed(&Actions::Down) && self.selected_index < self.labels.len() - 1 { 
            self.selected_index += 1;
        }

        if state.action_pressed(&Actions::Accept) {
            match self.selected_index {
                0 => return SceneLoad::Load(Box::new(crate::gust::level::Level::new())),
                1 => return SceneLoad::None,
                2 => return SceneLoad::Unload,
                _ => panic!("Invalid selection index"),
            }
        }
        SceneLoad::None
    }
    fn render(&self, graphics: &mut crate::Graphics) {
        const PADDING: f32 = 10.;
        let mut y = 0.;

        for (i, l) in self.labels.iter().enumerate() {
            let color = if self.selected_index == i { speedy2d::color::Color::RED } else { speedy2d::color::Color::WHITE };

            let x = l.align_h(&crate::graphics::screen_rect(), HorizontalAlignment::Center);
            l.render(graphics, crate::V2::new(x, y), color);
            y += l.size().y + PADDING;
        }
        
    }
}
impl MessageHandler for MainMenu {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages) {}
}