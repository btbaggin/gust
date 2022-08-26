use super::{Message, MessageKind, MessageAddress, TypeAddress};
use crate::entity::EntityHandle;
use std::collections::VecDeque;

pub type SharedMessageBus = std::rc::Rc<std::cell::RefCell<MessageBus>>;
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
    pub fn send_to_group(&mut self, kind: MessageKind, recipient: TypeAddress) {
        let message = Message {
            recipient: Some(MessageAddress::Type(recipient)),
            kind
        };
        self.messages.push_back(message);
    }

    pub fn send_to_entity(&mut self, kind: MessageKind, recipient: EntityHandle) {
        let message = Message {
            recipient: Some(MessageAddress::Entity(recipient)),
            kind
        };
        self.messages.push_back(message);
    }

    pub fn pop_message(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    pub fn register(&mut self, handler: impl MessageHandler, kind: MessageKind) {
        //TODO!!!!!
    }

    pub fn unregister(&mut self, handler: impl MessageHandler, kind: MessageKind) {
        //TODO!!!!!
    }
}