use serde::Deserialize;

pub const CONFIG_FILENAME: &str = "Config.toml";

#[derive(Clone, Default, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
}

#[derive(Clone, Default, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Default, Deserialize)]
pub struct LogConfig {}


