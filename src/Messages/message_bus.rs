use super::{Message, MessageType, MessageAddress};
use std::collections::VecDeque;

pub struct MessageBus {
    messages: VecDeque<Message>,
    handlers: Vec<Message>,
}
impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            messages: VecDeque::new(),
            handlers: Vec::new(),
        }
    }
    pub fn send(&mut self, message_type: MessageType) {
        let message = Message {
            recipient: None,
            message_type
        };
        self.messages.push_back(message);
    }
    pub fn send_to(&mut self, message_type: MessageType, recipient: MessageAddress) {
        let message = Message {
            recipient: Some(recipient),
            message_type
        };
        self.messages.push_back(message);
    }

    pub fn pop_message(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }
}