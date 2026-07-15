use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing::warn;

pub const SYNC_STARTED: &str = "sync_started";
pub const SYNC_PROGRESS: &str = "sync_progress";
pub const SYNC_COMPLETED: &str = "sync_completed";
pub const DATABASE_UPDATED: &str = "database_updated";
pub const SETTINGS_CHANGED: &str = "settings_changed";

pub fn emit<S: Serialize + Clone>(app: &AppHandle, event: &str, payload: S) {
    if let Err(err) = app.emit(event, payload) {
        warn!(%event, %err, "failed to emit event");
    }
}
