use crate::ui_comm::application::message_receiver::MessageReceiver;
use crate::ui_comm::infrastructure::message_receiver_impl::MessageReceiverImpl;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

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


#[no_mangle]
pub extern "C" fn send_message_to_rust(message_ptr: *const c_char) {
    let message = convert_c_string_to_string(message_ptr);

    unsafe {
        if let Some(ref sender) = FLUTTER_SENDER {
            let _ = sender.send(message);
        }
    }
}

fn convert_c_string_to_string(c_string: *const c_char) -> String {
    if c_string.is_null() {
        return "".to_string();
    }
    unsafe {
        CStr::from_ptr(c_string)
            .to_string_lossy()
            .into_owned()
    }
}