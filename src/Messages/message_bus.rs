use super::{Messages, MessageHandler};
use std::collections::VecDeque;

pub type SharedMessageBus = std::rc::Rc<std::cell::RefCell<MessageBus>>;
pub struct MessageBus {
    messages: VecDeque<Messages>,
    recipients: [Option<Vec<*mut dyn MessageHandler>>; 256]
}
impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            messages: VecDeque::new(),
            recipients: crate::utils::init_optional_array_to_blank(),
        }
    }
    pub fn send(&mut self, message: Messages) {
        self.messages.push_back(message);
    }

    pub fn process_messages(&mut self) {
        while let Some(m) = self.messages.pop_front() {
            let kind = super::raw_kind(&m);

            if let Some(recipients) = &mut self.recipients[kind as usize] {
                for r in &mut recipients.iter() {
                    let handler: &mut dyn MessageHandler = unsafe { (*r).as_mut().expect("Message handler was not property remove from message bus") };
                    handler.process(&m);
                }
            }
        }
    }

    pub fn register(&mut self, handler: &mut (impl MessageHandler + 'static), kind: u8) {
        if self.recipients[kind as usize].is_none() {
            self.recipients[kind as usize] = Some(vec!());
        }
        let recipients = &mut self.recipients[kind as usize].as_mut().unwrap();
        recipients.push(handler as *mut dyn MessageHandler);
    }

    pub fn unregister(&mut self, handler: &impl MessageHandler, kind: u8) {
        if let Some(recipients) = &mut self.recipients[kind as usize] {
            MessageBus::_unregister(handler, recipients);
        }
    }

    pub fn unregister_all(&mut self, handler: &impl MessageHandler) {
        for handlers in &mut self.recipients.iter_mut().flatten() {
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