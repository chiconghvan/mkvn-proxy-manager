use tauri::{AppHandle, State};

use crate::{models::{ProxyRow, SyncResult}, state::AppState, sync::engine::SyncEngine};

#[tauri::command]
pub async fn sync_all(state: State<'_, AppState>, app: AppHandle) -> Result<SyncResult, String> {
    SyncEngine::sync_all(&state, &app).await.map_err(Into::into)
}

#[tauri::command]
pub async fn get_proxy_rows(state: State<'_, AppState>) -> Result<Vec<ProxyRow>, String> {
    state.db.get_proxy_rows().map_err(Into::into)
}

#[tauri::command]
pub async fn get_cached_rows(state: State<'_, AppState>) -> Result<Vec<ProxyRow>, String> {
    state.db.get_proxy_rows().map_err(Into::into)
}
