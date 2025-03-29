use std::fs;
use model_manager::{DynamicValue, DynamicValueImpl, JsonToDynamicValueConverter, ModelManager, ModelManagerImpl};
use crate::ui_config::application::ui_config::UIConfig;

type SelectedDynamicValue = DynamicValueImpl;
type SelectedModelManager = ModelManagerImpl<SelectedDynamicValue>;

pub async fn load(){
    let mut manager: SelectedModelManager = SelectedModelManager::new();
    println!("Manager creado.");

    let converter = JsonToDynamicValueConverter;
    let mut service = UIConfig::new(manager, converter);
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
}