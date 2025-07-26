
pub struct CoreConfig {
    QdrantConfig,
    APIConfig
}

impl CoreConfig {
    pub fn to_config_vec<T>(&self) -> Vec<T> {
        match self {
            CoreConfig::QdrantConfig => vec![],
            CoreConfig::APIConfig => vec![]
        }
    }
}