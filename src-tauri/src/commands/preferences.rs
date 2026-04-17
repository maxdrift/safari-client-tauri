use std::fs;
use std::path::PathBuf;

use crate::commands::persistence::safari_client_dir;
use crate::models::UserPreferences;

fn preferences_path() -> Result<PathBuf, String> {
    Ok(safari_client_dir()?.join("preferences.json"))
}

pub fn load_preferences() -> Result<UserPreferences, String> {
    let path = preferences_path()?;
    if !path.exists() {
        return Ok(UserPreferences::default());
    }
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

pub fn save_preferences(prefs: &UserPreferences) -> Result<(), String> {
    let path = preferences_path()?;
    let json = serde_json::to_string_pretty(prefs).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
