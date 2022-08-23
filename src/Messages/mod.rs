mod message_bus;
pub use message_bus::MessageBus;
use crate::entity::{EntityHandle, EntityBehavior};

pub type TypeAddress = std::any::TypeId;
pub enum MessageAddress {
    Entity(EntityHandle),
    Type(TypeAddress),
}

pub trait MessageHandler {
    fn address(&self) -> TypeAddress;
    fn process(&mut self, message: &Message, message_bus: &mut MessageBus);
}

#[macro_export]
macro_rules! set_address {
    ($name:ident) => { fn address(&self) -> crate::messages::TypeAddress { std::any::TypeId::of::<$name>() } }
}
#[macro_export]
macro_rules! get_address { 
    ($name:ty) => { std::any::TypeId::of::<$name>() }
}

pub enum MessageKind {
    CreateEntity(Box<dyn EntityBehavior>),
}

pub struct Message {
    recipient: Option<MessageAddress>,
    kind: MessageKind,
}
impl Message {
    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
}
