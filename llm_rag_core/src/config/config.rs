use std::any::Any;
use crate::config::qdrant::{QdrantConfig, QdrantLoggerConfig, QdrantStorageConfig};
use crate::config::api::APIConfig;
use crate::exceptions::RagExceptions::ErrorWhileReadConfig;

pub struct CoreConfig {
    pub qdrant_config: QdrantConfig,
    pub api_config: APIConfig,
}

impl CoreConfig {
    pub fn config_to_vec<'a>(&'a self) -> Result<Vec<Box<dyn Any + 'a>>, String> {
        let vec: Vec<Box<dyn Any>> = vec![
            Box::new(&self.qdrant_config.logger_config),
            Box::new(&self.qdrant_config.storage_config),
            Box::new(&self.qdrant_config.service_config),
            Box::new(&self.qdrant_config.cluster_config),
        ];
        Ok(vec)
    }
}
