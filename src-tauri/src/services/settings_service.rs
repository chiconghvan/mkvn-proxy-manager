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
