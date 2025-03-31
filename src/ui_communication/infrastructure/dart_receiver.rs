use crate::ui_communication::application::message_receiver::MessageReceiver;
use crate::ui_communication::infrastructure::message_receiver_impl::MessageReceiverImpl;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

// Global sender to receive messages from Flutter
static mut FLUTTER_SENDER: Option<Sender<String>> = None;

pub fn start_message_listener() {
    let message_receiver = MessageReceiverImpl;

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    unsafe {
        FLUTTER_SENDER = Some(tx);
    }
    while let Ok(message) = rx.recv() {
        message_receiver.receive(message);
    }
}

/// FFI function that Flutter calls to send messages to Rust (non-blocking)
#[no_mangle]
pub extern "C" fn send_message_to_rust(message_ptr: *const c_char) {
    if message_ptr.is_null() {
        return;
    }

    // Convert C string to Rust String
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_string_lossy()
        .into_owned();

    // Send message to Rust thread
    unsafe {
        if let Some(ref sender) = FLUTTER_SENDER {
            let _ = sender.send(message);
        }
    }
}
