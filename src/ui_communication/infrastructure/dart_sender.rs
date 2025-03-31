use std::ffi::CString;
use std::sync::{Mutex, OnceLock};
use crate::ui_communication::infrastructure::dart_post_c_object_sender::{DartCObject, DartCObjectValue, DART_POST_COBJECT};

static INSTANCE: OnceLock<DartSender> = OnceLock::new();

#[derive(Debug)]
pub struct DartSender {
    dart_send_port: i64,
}

impl DartSender {
    /// Constructor privado para forzar el uso de `initialize`
    fn new(dart_send_port: i64) -> Self {
        Self { dart_send_port }
    }

    /// Obtiene la instancia única del Singleton
    pub fn get_instance() -> Result<&'static DartSender, &'static str> {
        INSTANCE.get().ok_or("Singleton no inicializado. Llama a `initialize` primero.")
    }

    /// Inicializa la instancia única
    pub fn initialize(dart_send_port: i64) -> Result<(), &'static str> {
        INSTANCE.set(Self::new(dart_send_port)).map_err(|_| "Singleton ya inicializado")
    }

    /// Envía un mensaje a Dart
    pub fn send_message(&self, message: &str) -> Result<(), &'static str> {
        let mensaje = CString::new(message).map_err(|_| "Error al convertir el mensaje a CString")?;
        let ptr_mensaje = mensaje.as_ptr() as *const u8;

        let mut c_object = DartCObject {
            typ: 5,  // 5 representa `String` en Dart
            value: DartCObjectValue { as_string: ptr_mensaje },
        };

        let dart_func = DART_POST_COBJECT.lock().unwrap();
        if let Some(post_cobject) = *dart_func {
            let success = post_cobject(self.dart_send_port, &mut c_object);
            println!("[Rust] Enviado: {}", message);
            if !success {
                return Err("Error al enviar mensaje a Flutter");
            }
            Ok(())
        } else {
            Err("DART_POST_COBJECT no está registrado")
        }
    }
}
