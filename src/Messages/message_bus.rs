use super::{Message, MessageKind, MessageAddress};
use std::collections::VecDeque;

pub struct MessageBus {
    messages: VecDeque<Message>,
}
impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            messages: VecDeque::new(),
        }
    }
    pub fn send(&mut self, kind: MessageKind) {
        let message = Message {
            recipient: None,
            kind
        };
        self.messages.push_back(message);
    }
    pub fn send_to(&mut self, kind: MessageKind, recipient: MessageAddress) {
        let message = Message {
            recipient: Some(recipient),
            kind
        };
        self.messages.push_back(message);
    }

    pub fn pop_message(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }
}