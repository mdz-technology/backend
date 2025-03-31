pub mod dart_messenger;
pub mod flutter_listener;

use std::thread;
use std::time::Duration;
use rand::Rng;

#[no_mangle]
pub extern "C" fn start_rust_thread(dart_send_port: i64) {
    thread::spawn(move || {
        let mut contador = 1;
        loop {
            dart_messenger::send_message_to_dart(dart_send_port, contador);
            contador += 1;
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        flutter_listener::start_message_listener();
    });
}








