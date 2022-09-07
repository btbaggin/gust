mod message_bus;
pub use message_bus::{MessageBus, SharedMessageBus};

pub enum Messages {
    EnemyGotToEnd(u32),
    EnemyKilled,
}


pub trait MessageHandler {
    fn process(&mut self, message: &Messages);
    fn register(&mut self, messages: &mut MessageBus);
    fn unregister(&self, messages: &mut MessageBus);
}

#[macro_export]
macro_rules! handle_messages {
    ($($message:expr),*) => { 
        fn register(&mut self, _messages: &mut crate::messages::MessageBus) {
            $(
                let kind = crate::messages::raw_kind(&$message);
                _messages.register(self, kind);
            )*
        }
        fn unregister(&self, messages: &mut crate::messages::MessageBus) {
            messages.unregister_all(self);
        }
    }
}

pub fn raw_kind(kind: &Messages) -> u8 {
    //SUUUUPER janky code to get the hidden tag field behind enums
    let parts = unsafe {
        core::slice::from_raw_parts(kind as *const _ as *const u8, std::mem::size_of_val(kind))
    };
    parts[0]
}