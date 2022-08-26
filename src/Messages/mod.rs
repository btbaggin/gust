mod message_bus;
use std::hash::Hash;
pub use message_bus::{MessageBus, SharedMessageBus};

#[derive(PartialEq, Eq, Hash)]
pub enum MessageKind {
    SpawnEnemy
}


pub trait MessageHandler {
    fn process(&mut self, message: &Message);
    fn register(&mut self, messages: &mut MessageBus);
    fn unregister(&self, messages: &mut MessageBus);
}

#[macro_export]
macro_rules! handle_messages {
    ($($message:path),*) => { 
        fn register(&mut self, _messages: &mut crate::messages::MessageBus) {
            $(_messages.register(self, $message);)*
        }
        fn unregister(&self, messages: &mut crate::messages::MessageBus) {
            messages.unregister_all(self);
        }
    }
}
pub struct Message {
    kind: MessageKind,
}
impl Message {
    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
}
