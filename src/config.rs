use rocket::figment::value::Map;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub min_connections: Option<u32>,
    pub max_connections: Option<u32>,
    pub connect_timeout: Option<u32>,
    pub idle_timeout: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub databases: Map<String, DatabaseConfig>,
}
