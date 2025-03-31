use crate::ui_communication::infrastructure::message_sender_impl::MessageSenderImpl;

pub trait MessageSender {
    fn new() -> Self;
    fn send(&self, message: &str);
}