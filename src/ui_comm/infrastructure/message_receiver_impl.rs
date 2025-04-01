use crate::ui_comm::application::message_receiver::MessageReceiver;

pub struct MessageReceiverImpl;

impl MessageReceiver for MessageReceiverImpl {
    fn receive(&self, message: String) {
        println!("[Rust] recibio: {}", message);
    }
}