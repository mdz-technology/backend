use crate::ui_communication::application::message_receiver::MessageReceiver;

pub struct MessageReceiverImpl;

impl MessageReceiver for MessageReceiverImpl {
    fn receive(&self, message: String) {
        println!("[Rust] Rust recibio: {}", message);
    }
}