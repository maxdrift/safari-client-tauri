use std::fs;
use std::path::PathBuf;

use crate::models::AppState;

/// Shared app data directory (`…/SafariClient`).
pub fn safari_client_dir() -> Result<PathBuf, String> {
    let base = dirs::data_local_dir().ok_or_else(|| "impossibile trovare la cartella dati".to_string())?;
    let dir = base.join("SafariClient");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn state_path() -> Result<PathBuf, String> {
    Ok(safari_client_dir()?.join("state.json"))
}

pub fn save_state(state: &AppState) -> Result<(), String> {
    let path = state_path()?;
    let json = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

pub fn load_state() -> Result<AppState, String> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(AppState::default());
    }
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}
