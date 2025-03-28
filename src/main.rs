use actix::System;
use model_manager::{DynamicValue, DynamicValueImpl, JsonToDynamicValueConverter, ModelManager, ModelManagerImpl};
use model_manager::DynamicValueConverter;
use std::fs;

type SelectedDynamicValue = DynamicValueImpl;
type SelectedModelManager = ModelManagerImpl<SelectedDynamicValue>;

fn main() {
    println!("Iniciando sistema Actix...");

    let system = System::new();

    system.block_on(async {
        let handle = actix::spawn(async {
            let mut manager: SelectedModelManager = SelectedModelManager::new();
            println!("Manager creado.");

            let converter = JsonToDynamicValueConverter;
            let mut service = Service::new(manager, converter);
            let file_path = "config/data.json";
            let json_str = fs::read_to_string(file_path).expect("Error al leer el archivo JSON");
            println!("Contenido del archivo JSON: {}", json_str);
            match service.load_json(&json_str).await {
                Ok(value) => {
                    println!("Loaded value: {:?}", value.get_type());
                },
                Err(err) => println!("Error: {}", err),
            }
            println!("Service creado.");
        });

        let _ = handle.await;
    });

    println!("Finalizando ejecución...");
}

pub struct Service<M, C>
where
    M: ModelManager + Send + Sync,
    C: for<'a> DynamicValueConverter<&'a str> + Send + Sync,
{
    model_manager: M,
    converter: C
}

impl<M, C> Service<M, C>
where
    M: ModelManager + Send + Sync,
    C: for<'a> DynamicValueConverter<&'a str, Output = M::Value> + Send + Sync,
{
    pub fn new(model_manager: M, converter: C) -> Self {
        Self {
            model_manager,
            converter
        }
    }

    pub async fn load_json(&mut self, data: &str) -> Result<M::Value, String> {
        let new_value = self.converter.convert(data)?;
        Ok(new_value)
        //self.model_manager.insert("algo".to_string(), None, new_value).await
    }
}