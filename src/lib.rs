pub mod ui_communication;

use std::thread;
use std::time::Duration;
use rand::Rng;
use ui_communication::infrastructure::dart_receiver;
use ui_communication::infrastructure::dart_sender::DartSender;
use crate::ui_communication::application::message_sender::MessageSender;
use crate::ui_communication::infrastructure::message_sender_impl::MessageSenderImpl;

#[no_mangle]
extern "C" fn start_rust_thread(dart_send_port: i64) {

    DartSender::initialize(dart_send_port);

    thread::spawn(move || {
        dart_receiver::start_message_listener();
    });

    thread::spawn(move || {
        let mut contador = 1;
        let messageSender = MessageSenderImpl::new();
        loop {
            let msg = format!("Mensaje #{} enviado", contador);
            messageSender.send(&msg);
            contador += 1;
            thread::sleep(Duration::from_millis(1000));
        }
    });
}









