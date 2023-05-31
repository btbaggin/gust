use crate::entity::{SceneBehavior, SceneLoad, EntityManager};
use crate::assets::{Sound, Sounds, SoundHandle, SoundStatus, Fonts};
use crate::job_system::ThreadSafeJobQueue;
use crate::messages::{Messages, MessageHandler, MessageContext};
use crate::ui::{Label, DockContainer};
use crate::input::Actions;
use crate::graphics::Color;

pub struct MainMenu {
    selected_index: usize,
    audio_handle: Option<SoundHandle>,
    labels: [&'static str; 3]
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu { 
            selected_index: 0,
            audio_handle: None,
            labels: ["new", "settings", "exit"],
        }
    }

    fn move_index(&mut self, up: bool, root: &mut crate::ui::Widget) {
        let l = root.find_mut::<Label>(self.labels[self.selected_index]).unwrap();
        l.set_color(Color::WHITE);

        if up {
            self.selected_index -= 1;
        } else {
            self.selected_index += 1;
        }

        let l = root.find_mut::<Label>(self.labels[self.selected_index]).unwrap();
        l.set_color(Color::RED);
    }
}
impl SceneBehavior for MainMenu {
    fn load(&mut self, queue: ThreadSafeJobQueue, _entities: &mut EntityManager, root: &mut crate::ui::Widget) {
        root.with_child(DockContainer::vertical(0.5, 0.5, None))
            .add_named_child(Label::new(String::from("New Game"), Fonts::Regular, 64., Color::RED), self.labels[0])
            .add_named_child(Label::new(String::from("Settings"), Fonts::Regular, 64., Color::WHITE), self.labels[1])
            .add_named_child(Label::new(String::from("Exit"), Fonts::Regular, 64., Color::WHITE), self.labels[2]);

        self.audio_handle = Some(Sound::repeat(&queue, Sounds::Piano));
    }

    fn unload(&mut self) {
        Sound::set_status(self.audio_handle.unwrap(), SoundStatus::Stopped);
    }

    fn update(&mut self, state: &mut crate::UpdateState, root: &mut crate::ui::Widget) -> SceneLoad {
        if state.action_pressed(Actions::Up) && self.selected_index > 0 { 
            self.move_index(true, root);
            crate::assets::Sound::play(&state.queue, Sounds::Shoot);
    }
        if state.action_pressed(Actions::Down) && self.selected_index < self.labels.len() - 1 { 
            self.move_index(false, root);
            crate::assets::Sound::play(&state.queue, Sounds::Shoot);
        }

        if state.action_pressed(Actions::Accept) {
            match self.selected_index {
                0 => return SceneLoad::Load(Box::new(crate::gust::level::Level::new())),
                1 => return SceneLoad::None,
                2 => return SceneLoad::Unload,
                _ => panic!("Invalid selection index"),
            }
        }
        SceneLoad::None
    }

    fn render(&self, _graphics: &mut crate::Graphics) { }
}
impl MessageHandler for MainMenu {
    crate::handle_messages!();
    fn process(&mut self, _message: &Messages, _context: &mut MessageContext) {}
}