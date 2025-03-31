use crate::ui_communication::infrastructure::dart_sender::DartSender;
use crate::ui_communication::application::message_sender::MessageSender;

pub struct MessageSenderImpl;
impl MessageSender for MessageSenderImpl {

    fn new() -> Self {
        MessageSenderImpl
    }

    fn send(&self, message: &str) {
        match DartSender::get_instance() {
            Ok(dartSender) => {
                if let Err(e) = dartSender.send_message(message) {
                    eprintln!("Error enviando mensaje: {}", e);
                }
            }
            Err(e) => eprintln!("Error obteniendo DartMessenger: {}", e),
        }
    }
}