use tauri::{AppHandle, State};

use crate::{models::AppSettings, services::settings_service, state::AppState};

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.read().clone())
}

#[tauri::command]
pub async fn save_settings(state: State<'_, AppState>, app: AppHandle, settings: AppSettings) -> Result<(), String> {
    settings_service::save_settings(&state, &app, settings).map_err(Into::into)
}
