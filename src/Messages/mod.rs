mod message_bus;
pub use message_bus::MessageBus;
use crate::entity::EntityHandle;

pub type TypeAddress = std::any::TypeId;
pub enum MessageAddress {
    Entity(EntityHandle),
    Type(TypeAddress),
}

pub trait MessageHandler {
    fn address(&self) -> TypeAddress;
    fn process(&mut self, message: &Message);
}

#[macro_export]
macro_rules! set_address {
    ($name:ident) => { fn address(&self) -> crate::messages::TypeAddress { std::any::TypeId::of::<$name>() } }
}
#[macro_export]
macro_rules! get_address { 
    ($name:ty) => { std::any::TypeId::of::<$name>() }
}

pub enum MessageType {
    Test
}

pub struct Message {
    recipient: Option<MessageAddress>,
    message_type: MessageType,
}

