// src/state.rs
use crate::tunnel::TunnelInfo;
use crate::TunnelMap;
use std::fs;

const STATE_FILE: &str = "tunnels.json";

pub async fn load_state() -> Vec<TunnelInfo> {
    match fs::read_to_string(STATE_FILE) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub async fn save_state(map: &TunnelMap) {
    let infos: Vec<_> = map
        .lock()
        .await
        .values()
        .map(|h| h.info.clone())
        .collect();
    if let Ok(json) = serde_json::to_string_pretty(&infos) {
        let _ = fs::write(STATE_FILE, json);
    }
}
