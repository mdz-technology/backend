use std::ffi::CString;
use std::os::raw::c_int;
use std::thread;
use std::time::Duration;

/// Tipo de la función `Dart_PostCObject_DL`
type DartPostCObjectFn = extern "C" fn(i64, *mut DartCObject) -> bool;

/// Definición de `DartCObject` (simplificada)
#[repr(C)]
pub struct DartCObject {
    pub typ: c_int,
    pub value: DartCObjectValue,
}

#[repr(C)]
pub union DartCObjectValue {
    pub as_string: *const u8,
}

/// Puntero global a `Dart_PostCObject_DL`
static mut DART_POST_COBJECT: Option<DartPostCObjectFn> = None;

/// Se llama desde Dart para registrar `Dart_PostCObject_DL`
#[no_mangle]
pub extern "C" fn register_dart_post_cobject(func: DartPostCObjectFn) {
    unsafe {
        DART_POST_COBJECT = Some(func);
    }
}

pub fn send_message_to_dart(dart_send_port: i64, index: i32) {
    let mensaje = CString::new(format!("Mensaje #{} enviado", index))
        .unwrap();
    let ptr_mensaje = mensaje.as_ptr() as *const u8; // ✅ Convertimos en puntero

    let mut c_object = DartCObject {
        typ: 5,  // ✅ 4 es `String` en Dart
        value: DartCObjectValue { as_string: ptr_mensaje },
    };

    unsafe {
        if let Some(post_cobject) = DART_POST_COBJECT {
            let success = post_cobject(dart_send_port, &mut c_object);
            println!("{}", format!("[Rust] Rust envio: #{}",format!("Mensaje #{} enviado", index)));
            if !success {
                eprintln!("Error al enviar mensaje a Flutter");
            }
        } else {
            eprintln!("DART_POST_COBJECT no está registrado");
        }
    }

    // 🔹 Liberamos la memoria después de enviar el mensaje
    /*unsafe {
        let _ = CString::from_raw(ptr_mensaje);
    }*/
}