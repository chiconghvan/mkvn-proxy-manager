use std::io::Write;
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

#[tauri::command]
pub async fn download_update(url: String) -> Result<String, String> {
    let temp_dir = std::env::temp_dir().join("mkvn-proxy-update");
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {e}"))?;

    let filename = url.rsplit('/').next().unwrap_or("update.exe");
    let dest = temp_dir.join(filename);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download: {e}"))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;

    let mut file =
        std::fs::File::create(&dest).map_err(|e| format!("Failed to create file: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub fn restart_application(installer_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let current_exe = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {e}"))?;
        let pid = std::process::id();
        let temp_dir = std::env::temp_dir().join("mkvn-proxy-update");
        std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {e}"))?;
        let script_path = temp_dir.join("restart.bat");

        let is_msi = installer_path.to_lowercase().ends_with(".msi");
        let install_cmd = if is_msi {
            format!(
                "start \"\" /wait \"{}\\System32\\msiexec.exe\" /i \"{}\" /quiet /norestart /promptrestart",
                std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string()),
                installer_path
            )
        } else {
            format!("start \"\" /wait \"{}\" /S /UPDATE", installer_path)
        };

        let script = format!(
            "@echo off\r\n:w\r\ntasklist /fi \"PID eq {}\" 2>nul | find \"{}\" >nul && (timeout /t 1 /nobreak >nul & goto w)\r\n{}\r\ntimeout /t 1 /nobreak >nul\r\nstart \"\" \"{}\"\r\ndel \"%~f0\"\r\n",
            pid, pid, install_cmd, current_exe.display()
        );

        std::fs::write(&script_path, &script)
            .map_err(|e| format!("Failed to write script: {e}"))?;

        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        std::process::Command::new("cmd")
            .args(["/C", script_path.to_str().unwrap()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("Failed to spawn updater: {e}"))?;

        std::thread::sleep(std::time::Duration::from_millis(500));
        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = installer_path;
        std::process::exit(0);
    }
}
