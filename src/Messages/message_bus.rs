use super::{Message, MessageKind, MessageHandler};
use std::collections::{VecDeque, HashMap};

pub type SharedMessageBus = std::rc::Rc<std::cell::RefCell<MessageBus>>;
pub struct MessageBus {
    messages: VecDeque<Message>,
    recipients: HashMap<MessageKind, Vec<*mut dyn MessageHandler>>,
}
impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            messages: VecDeque::new(),
            recipients: HashMap::new(),
        }
    }
    pub fn send(&mut self, kind: MessageKind) {
        let message = Message { kind };
        self.messages.push_back(message);
    }

    pub fn process_messages(&mut self) {
        while let Some(m) = self.messages.pop_front() {
            if let Some(recipients) = self.recipients.get_mut(&m.kind()) {
                for r in &mut recipients.iter() {
                    let handler: &mut dyn MessageHandler = unsafe { (*r).as_mut().expect("Message handler was not property remove from message bus") };
                    handler.process(&m);
                }
            }
        }
    }

    pub fn register(&mut self, handler: &mut (impl MessageHandler + 'static), kind: MessageKind) {
        let recipients = self.recipients.entry(kind).or_insert(vec!());
        recipients.push(handler as *mut dyn MessageHandler);
    }

    pub fn unregister(&mut self, handler: &impl MessageHandler, kind: MessageKind) {
        if let Some(mut recipients) = self.recipients.get_mut(&kind) {
            MessageBus::_unregister(handler, &mut recipients);
        }
    }

    pub fn unregister_all(&mut self, handler: &impl MessageHandler) {
        for handlers in self.recipients.values_mut() {
            MessageBus::_unregister(handler, handlers);
        }
    }

    fn _unregister(handler: &impl MessageHandler, handlers: &mut Vec<*mut dyn MessageHandler>) {
        let mut remove = None;
        for (i, r) in handlers.iter().enumerate() {
            let r = unsafe { (*r).as_ref().unwrap() };
            if std::ptr::eq(r, handler) {
                remove = Some(i);
            }
        }

        if let Some(i) = remove {
            handlers.remove(i);
        }
    }
}