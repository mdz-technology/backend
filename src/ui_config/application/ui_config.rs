use model_manager::{DynamicValueConverter, ModelManager};

pub struct UIConfig<M, C>
where
    M: ModelManager + Send + Sync,
    C: for<'a> DynamicValueConverter<&'a str> + Send + Sync,
{
    model_manager: M,
    converter: C
}

impl<M, C> UIConfig<M, C>
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