use tauri::{AppHandle, State};

use crate::{events::{self, DATABASE_UPDATED}, services::proxy_service, state::AppState};

#[tauri::command]
pub async fn buy_proxy(state: State<'_, AppState>, app: AppHandle, product_id: i64, quantity: i64, renewal: bool, note: String) -> Result<Vec<String>, String> {
    let codes = proxy_service::buy_proxy(&state, product_id, quantity, renewal, note).await.map_err(String::from)?;
    events::emit(&app, DATABASE_UPDATED, ());
    Ok(codes)
}

#[tauri::command]
pub async fn renew_order(state: State<'_, AppState>, order_code: String, months: i64) -> Result<(), String> {
    proxy_service::renew_order(&state, order_code, months).await.map_err(Into::into)
}

#[tauri::command]
pub async fn toggle_renewal(state: State<'_, AppState>, order_code: String, enable: bool) -> Result<(), String> {
    proxy_service::toggle_renewal(&state, order_code, enable).await.map_err(Into::into)
}
