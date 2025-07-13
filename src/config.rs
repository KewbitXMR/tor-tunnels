use std::fs::{self, File};
use std::collections::HashMap;
use std::io::Write;
use serde::{Serialize, Deserialize};

const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigInfo {
    pub listen_addr: String,
    pub socks5_proxy_host: String,
    pub socks5_proxy_port: u16,
}

/// A HashMap of named configurations: `{ "default": ConfigInfo, ... }`
pub type ConfigMap = HashMap<String, ConfigInfo>;

pub async fn load_config() -> ConfigMap {
    if !std::path::Path::new(CONFIG_FILE).exists() {
        println!("Config file not found. Creating default config.json...");

        let mut default_map = HashMap::new();
        default_map.insert(
            "default".to_string(),
            ConfigInfo {
                listen_addr: "127.0.0.1:8080".to_string(),
                socks5_proxy_host: "127.0.0.1".to_string(),
                socks5_proxy_port: 9050,
            },
        );

        if let Ok(json) = serde_json::to_string_pretty(&default_map) {
            let mut file = File::create(CONFIG_FILE).expect("Unable to create config file");
            file.write_all(json.as_bytes()).expect("Unable to write config file");
            println!("Default config.json created.");
        }
    }

    match fs::read_to_string(CONFIG_FILE) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|e| {
            eprintln!("Failed to parse config.json: {e}");
            HashMap::new()
        }),
        Err(e) => {
            eprintln!("Failed to read config.json: {e}");
            HashMap::new()
        }
    }
}