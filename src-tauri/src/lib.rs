mod commands;
mod imaging;
mod models;

use commands::export;
use commands::images;
use commands::persistence;
use commands::species;
use models::{AppState, SlideDto, SpeciesDto, ThumbnailPaths};

#[tauri::command]
fn load_species_catalog_cmd() -> Result<Vec<SpeciesDto>, String> {
    species::load_species_catalog()
}

#[tauri::command]
fn save_app_state_cmd(state: AppState) -> Result<(), String> {
    persistence::save_state(&state)
}

#[tauri::command]
fn load_app_state_cmd() -> Result<AppState, String> {
    persistence::load_state()
}

#[tauri::command]
fn restore_slides_cmd() -> Result<Vec<SlideDto>, String> {
    let st = persistence::load_state()?;
    images::ensure_previews_for_persisted(st.slides)
}

#[tauri::command]
fn load_slides_from_paths_cmd(paths: Vec<String>) -> Result<Vec<SlideDto>, String> {
    images::load_slides_from_paths(paths)
}

#[tauri::command]
fn regenerate_thumbnails_cmd(path: String, transform_id: u8) -> Result<ThumbnailPaths, String> {
    images::regenerate_thumbnails(path, transform_id)
}

#[tauri::command]
fn remove_slide_cache_cmd(filename: String) {
    images::remove_slide_cache(filename);
}

#[tauri::command]
fn apply_transform_action_cmd(current_id: u8, action: String) -> Result<u8, String> {
    images::apply_transform_action(current_id, &action)
}

#[tauri::command]
fn export_csv_cmd(slides: Vec<SlideDto>, path: String) -> Result<(), String> {
    export::export_csv(&slides, std::path::Path::new(&path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_species_catalog_cmd,
            save_app_state_cmd,
            load_app_state_cmd,
            restore_slides_cmd,
            load_slides_from_paths_cmd,
            regenerate_thumbnails_cmd,
            remove_slide_cache_cmd,
            apply_transform_action_cmd,
            export_csv_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
