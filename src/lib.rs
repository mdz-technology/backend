use std::ffi::CStr;
use std::os::raw::{c_void, c_int};
use std::thread;
use std::time::Duration;
use std::ptr;

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

/// Función para iniciar un hilo de Rust que enviará un mensaje a Dart.
#[no_mangle]
pub extern "C" fn start_rust_thread(send_port: i64) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        let mensaje = "Mensaje desde Ru__st\0";
        let mut c_object = DartCObject {
            typ: 4,  // Suponiendo que 4 es String en Dart
            value: DartCObjectValue {
                as_string: mensaje.as_ptr(),
            },
        };

        unsafe {
            if let Some(post_cobject) = DART_POST_COBJECT {
                let success = post_cobject(send_port, &mut c_object);
                if !success {
                    eprintln!("Error al enviar mensaje a Flutter");
                }
            } else {
                eprintln!("DART_POST_COBJECT no está registrado");
            }
        }
    });
}

/// Recibe un mensaje desde Flutter
#[no_mangle]
pub extern "C" fn send_message_from_flutter(message: *const u8) {
    unsafe {
        if message.is_null() {
            eprintln!("Mensaje nulo desde Flutter");
            return;
        }
        let c_str = CStr::from_ptr(message as *const i8);
        match c_str.to_str() {
            Ok(texto) => println!("Mensaje recibido de Flutter: {}", texto),
            Err(e) => eprintln!("Error interpretando mensaje: {}", e),
        }
    }
}
