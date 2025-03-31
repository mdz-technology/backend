use crate::ui_communication::infrastructure::message_sender_impl::MessageSenderImpl;

pub trait MessageSender {
    fn send(&self, message: &str);
    fn new() -> Self;
}