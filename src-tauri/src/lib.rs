mod prayer;
mod tray;

use prayer::{fetch_prayer_times, PrayerConfig, PrayerTimesResult};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

struct AppState {
    config: Mutex<PrayerConfig>,
}

fn config_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("no app data dir")
        .join("config.json")
}

fn load_config(app: &tauri::AppHandle) -> PrayerConfig {
    let path = config_path(app);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_config(app: &tauri::AppHandle, config: &PrayerConfig) {
    let path = config_path(app);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = std::fs::write(path, json);
    }
}

#[tauri::command]
async fn get_prayer_times(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<PrayerTimesResult, String> {
    let config = state.config.lock().unwrap().clone();
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to access app data directory: {}", e))?;
    fetch_prayer_times(&config, &app_data_dir).await
}

#[tauri::command]
async fn update_config(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    config: PrayerConfig,
) -> Result<PrayerTimesResult, String> {
    {
        let mut current = state.config.lock().unwrap();
        *current = config;
    }
    let config = state.config.lock().unwrap().clone();
    save_config(&app, &config);
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to access app data directory: {}", e))?;
    fetch_prayer_times(&config, &app_data_dir).await
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, AppState>) -> Result<PrayerConfig, String> {
    Ok(state.config.lock().unwrap().clone())
}

#[tauri::command]
async fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn play_sound(sound_name: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let sound_path = format!("/System/Library/Sounds/{}.aiff", sound_name);
        Command::new("afplay")
            .arg(&sound_path)
            .spawn()
            .map_err(|e| format!("Failed to play sound: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn set_tray_title(app: tauri::AppHandle, title: String) {
    if let Some(tray) = app.tray_by_id("prayer-tray") {
        let _ = tray.set_title(Some(title.as_str()));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let config = load_config(app.handle());
            app.manage(AppState {
                config: Mutex::new(config),
            });
            tray::create_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_prayer_times,
            update_config,
            get_config,
            quit_app,
            play_sound,
            set_tray_title
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
