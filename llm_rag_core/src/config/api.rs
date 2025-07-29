use std::time::Duration;

pub struct APIConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
    pub proxies: Vec<String>
}