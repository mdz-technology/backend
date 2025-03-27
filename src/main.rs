use actix::System;
use model_manager::{DynamicValue, DynamicValueImpl, ModelManager, ModelManagerImpl};

type SelectedDynamicValue = DynamicValueImpl; // Cambia esto por otra implementación si es necesario
type SelectedModelManager = ModelManagerImpl<SelectedDynamicValue>; // También se puede cambiar

fn main() {
    println!("Iniciando sistema Actix...");

    let system = System::new();

    system.block_on(async {
        let handle = actix::spawn(async {
            let mut manager: SelectedModelManager = SelectedModelManager::new();
            println!("Manager creado.");

            let mut service = Service::new(manager);
            println!("Service creado.");

            let result = service.load_json("data").await;
            println!("Resultado: {:?}", result.ok());
        });

        let _ = handle.await;
    });

    println!("Finalizando ejecución...");
}

struct Service<M: ModelManager<Value = V> + Send + Sync, V: DynamicValue + Send + Sync> {
    model_manager: M,
    _marker: std::marker::PhantomData<V>,
}

impl<M: ModelManager<Value = V> + Send + Sync, V: DynamicValue + Send + Sync> Service<M, V> {
    fn new(model_manager: M) -> Self {
        Self {
            model_manager,
            _marker: std::marker::PhantomData,
        }
    }

    async fn load_json(&mut self, data: &str) -> Result<V, String> {
        let new_value = V::new_object(); // Ahora usamos el trait `DynamicValue`
        self.model_manager.insert("algo".to_string(), None, new_value).await
    }
}