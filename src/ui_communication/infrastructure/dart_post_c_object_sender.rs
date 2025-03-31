use std::os::raw::c_int;
use std::sync::{Mutex};

type DartPostCObjectFn = extern "C" fn(i64, *mut DartCObject) -> bool;

#[repr(C)]
pub struct DartCObject {
    pub typ: c_int,
    pub value: DartCObjectValue,
}

#[repr(C)]
pub union DartCObjectValue {
    pub as_string: *const u8,
}

pub static DART_POST_COBJECT: Mutex<Option<DartPostCObjectFn>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn register_dart_post_cobject(func: DartPostCObjectFn) {
    let mut dart_func = DART_POST_COBJECT.lock().unwrap();
    *dart_func = Some(func);
}