use tracing::info;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

pub async fn buy_proxy(state: &AppState, product_id: i64, quantity: i64, renewal: bool, note: String) -> AppResult<Vec<String>> {
    let client = state.mkvn_client().ok_or_else(|| AppError::NotConfigured("MKVN token is empty".to_string()))?;
    let mut codes = Vec::new();
    for _ in 0..quantity.max(0) {
        codes.push(client.buy(product_id, renewal, &note).await?);
    }
    Ok(codes)
}

pub async fn renew_order(state: &AppState, order_code: String, months: i64) -> AppResult<()> {
    info!(order_code, months, "renew_order called");
    let client = state.mkvn_client().ok_or_else(|| AppError::NotConfigured("MKVN token is empty".to_string()))?;
    let result = client.renew_plus(&order_code, months).await;
    match &result {
        Ok(_) => info!(order_code, months, "renew_order succeeded"),
        Err(e) => tracing::warn!(order_code, months, error = %e, "renew_order failed"),
    }
    result
}

pub async fn toggle_renewal(state: &AppState, order_code: String, enable: bool) -> AppResult<()> {
    let client = state.mkvn_client().ok_or_else(|| AppError::NotConfigured("MKVN token is empty".to_string()))?;
    client.renewal_on_off(&order_code, enable).await
}
