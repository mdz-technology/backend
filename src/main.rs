mod ui_config;

use actix::System;
/*use model_manager::{DynamicValue, DynamicValueImpl, JsonToDynamicValueConverter, ModelManager, ModelManagerImpl};
use model_manager::DynamicValueConverter;*/
use std::fs;
use crate::ui_config::application::ui_config::UIConfig;
use crate::ui_config::infrastructure::ui_config_impl::load;

fn main() {
    println!("Iniciando sistema Actix...");

    let system = System::new();

    system.block_on(async {
        let handle = actix::spawn(async {
            load().await;
        });

        let _ = handle.await;
    });

    println!("Finalizando ejecución...");
}

/*
ui_config

*/