use crate::V2;
use crate::input::{Input, Actions};
use crate::messages::{SharedMessageBus, MessageKind};
use crate::job_system::ThreadSafeJobQueue;
use crate::entity::EntityManager;

pub struct UpdateState<'a> {
    pub delta_time: f32,
    input: &'a Input,
    pub message_bus: SharedMessageBus,
    pub queue: ThreadSafeJobQueue,
    pub entities: &'a mut EntityManager
}
impl<'a> UpdateState<'a> {
    pub fn new(delta_time: f32, input: &'a Input, message_bus: SharedMessageBus,
               queue: ThreadSafeJobQueue, entities: &'a mut EntityManager) -> UpdateState<'a> {
        UpdateState { delta_time, input, message_bus, queue, entities }
    }
    pub fn send_message(&self, kind: MessageKind) {
        let mut m = self.message_bus.borrow_mut();
        m.send(kind);
    }
    pub fn action_down(&self, action: &Actions) -> bool {
       self.input.action_down(action)
    }
    pub fn action_up(&self, action: &Actions) -> bool {
        self.input.action_up(action)
    }
    pub fn action_pressed(&self, action: &Actions) -> bool {
        self.input.action_pressed(action)
    }
    pub fn action_released(&self, action: &Actions) -> bool {
        self.input.action_released(action)
    }
    pub fn mouse_pos(&self) -> V2 {
        self.input.mouse_pos()
    }
}