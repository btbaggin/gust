use crate::V2;
use crate::input::{Input, Actions};
use crate::messages::{SharedMessageBus, Messages};
use crate::job_system::ThreadSafeJobQueue;
use crate::entity::EntityManager;
use crate::physics::QuadTree;

pub struct UpdateState<'a> {
    pub delta_time: f32,
    input: &'a Input,
    pub message_bus: SharedMessageBus,
    pub queue: ThreadSafeJobQueue,
    pub entities: &'a mut EntityManager,
    pub quad_tree: &'a QuadTree,
}
impl<'a> UpdateState<'a> {
    pub fn new(delta_time: f32, input: &'a Input, message_bus: SharedMessageBus,
               queue: ThreadSafeJobQueue, entities: &'a mut EntityManager,
               quad_tree: &'a QuadTree) -> UpdateState<'a> {
        UpdateState { delta_time, input, message_bus, queue, entities, quad_tree }
    }
    pub fn send_message(&self, kind: Messages) {
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