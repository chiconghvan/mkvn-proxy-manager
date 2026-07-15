use tauri::State;

use crate::{models::AppUpdateInfo, state::AppState, updater};

#[tauri::command]
pub async fn check_for_updates(state: State<'_, AppState>) -> Result<AppUpdateInfo, String> {
    updater::check_for_updates(&state.http).await
}

#[tauri::command]
pub fn get_app_version() -> String {
    updater::current_version()
}
