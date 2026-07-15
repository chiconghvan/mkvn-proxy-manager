pub mod api;
pub mod commands;
pub mod database;
pub mod error;
pub mod events;
pub mod managers;
pub mod matcher;
pub mod models;
pub mod services;
pub mod state;
pub mod sync;

use std::path::PathBuf;

use tauri::Manager;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{commands::*, services::settings_service, state::AppState};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup_logging(app.handle());
            let db_path = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("proxies.db");
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let state = AppState::new(db_path)?;
            let _ = settings_service::load_settings(&state);
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            sync_all,
            get_proxy_rows,
            get_cached_rows,
            buy_proxy,
            renew_order,
            toggle_renewal,
            get_products,
            get_balance,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logging(app: &tauri::AppHandle) {
    let log_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from(".")).join("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    let file_appender = tracing_appender::rolling::daily(log_dir, "mkvn-proxy-manager.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().with_writer(non_blocking))
        .try_init();
}
