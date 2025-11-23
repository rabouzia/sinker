use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
struct BlockingConfig {
    pub enabled: bool,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
struct LoggingConfig {
    pub level: String,
    pub file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub blocking: BlockingConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
