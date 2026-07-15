use tauri::State;

use crate::{error::AppError, models::{Balance, Product}, state::AppState};

#[tauri::command]
pub async fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
    let client = state.mkvn_client().ok_or_else(|| AppError::NotConfigured("MKVN token is empty".to_string())).map_err(String::from)?;
    let products = client.get_products().await.map_err(String::from)?;
    state.db.upsert_products(&products).map_err(String::from)?;
    Ok(products)
}

#[tauri::command]
pub async fn get_balance(state: State<'_, AppState>) -> Result<Balance, String> {
    let client = state.mkvn_client().ok_or_else(|| AppError::NotConfigured("MKVN token is empty".to_string())).map_err(String::from)?;
    client.get_balance().await.map_err(Into::into)
}
