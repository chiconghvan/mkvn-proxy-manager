use tauri::AppHandle;

use crate::{
    error::AppResult,
    events::{self, SETTINGS_CHANGED},
    models::AppSettings,
    state::AppState,
};

pub fn load_settings(state: &AppState) -> AppResult<AppSettings> {
    let mut settings = AppSettings::default();
    if let Some(value) = state.db.get_setting("settings_json")? {
        settings = serde_json::from_str(&value).unwrap_or_default();
    }
    if settings.gpm_standard_url.trim().is_empty() {
        settings.gpm_standard_url = "http://127.0.0.1:19995/api/v3".to_string();
    }
    if settings.gpm_global_url.trim().is_empty() {
        settings.gpm_global_url = "http://127.0.0.1:9495/api/v1".to_string();
    }
    if settings.donut_url.trim().is_empty() {
        settings.donut_url = "http://127.0.0.1:10108/v1".to_string();
    }
    state.rebuild_clients(&settings);
    Ok(settings)
}

pub fn save_settings(state: &AppState, app: &AppHandle, settings: AppSettings) -> AppResult<()> {
    let json = serde_json::to_string_pretty(&settings)?;
    state.db.set_setting("settings_json", &json)?;
    state.rebuild_clients(&settings);
    events::emit(app, SETTINGS_CHANGED, settings);
    Ok(())
}
