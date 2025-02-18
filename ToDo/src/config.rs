use serde::Deserialize;
use std::{fs, sync::OnceLock};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub storage_path: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn load_config() {
    let config_path = "config/user-config.toml";
    let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config");

    CONFIG.set(config).expect("Config already initialized");
}

pub fn get_storage_path() -> String {
    CONFIG.get().expect("Config not loaded").storage_path.clone()
}
