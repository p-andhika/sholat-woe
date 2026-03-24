mod prayer;
mod tray;

use prayer::{fetch_prayer_times, PrayerConfig, PrayerTimesResult};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

struct CachedPrayers {
    date: String,
    data: PrayerTimesResult,
}

struct AppState {
    config: Mutex<PrayerConfig>,
    prayer_cache: Mutex<Option<CachedPrayers>>,
    notified_today: Mutex<(String, HashSet<String>)>,
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
    let result = fetch_prayer_times(&config, &app_data_dir).await?;

    // Update prayer cache for the background notification checker
    {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut cache = state.prayer_cache.lock().unwrap();
        *cache = Some(CachedPrayers {
            date: today,
            data: result.clone(),
        });
    }

    Ok(result)
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
    let result = fetch_prayer_times(&config, &app_data_dir).await?;

    // Update prayer cache
    {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut cache = state.prayer_cache.lock().unwrap();
        *cache = Some(CachedPrayers {
            date: today,
            data: result.clone(),
        });
    }

    Ok(result)
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, AppState>) -> Result<PrayerConfig, String> {
    Ok(state.config.lock().unwrap().clone())
}

#[tauri::command]
async fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

fn play_sound_internal(sound_name: &str) {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let sound_path = format!("/System/Library/Sounds/{}.aiff", sound_name);
        let _ = Command::new("afplay").arg(&sound_path).spawn();
    }
}

#[tauri::command]
fn play_sound(sound_name: String) -> Result<(), String> {
    play_sound_internal(&sound_name);
    Ok(())
}

#[tauri::command]
fn set_tray_title(app: tauri::AppHandle, title: String) {
    if let Some(tray) = app.tray_by_id("prayer-tray") {
        let _ = tray.set_title(Some(title.as_str()));
    }
}

fn start_notification_checker(app: tauri::AppHandle) {
    use chrono::Timelike;
    use tauri::Emitter;
    use tauri_plugin_notification::NotificationExt;

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));

            let state = app.state::<AppState>();
            let config = state.config.lock().unwrap().clone();
            let now = chrono::Local::now();
            let today = now.format("%Y-%m-%d").to_string();

            // Refresh cache if stale or empty
            let needs_fetch = {
                let cache = state.prayer_cache.lock().unwrap();
                match cache.as_ref() {
                    Some(c) => c.date != today,
                    None => true,
                }
            };

            if needs_fetch {
                if let Ok(dir) = app.path().app_data_dir() {
                    if let Ok(data) = tauri::async_runtime::block_on(
                        fetch_prayer_times(&config, &dir),
                    ) {
                        let mut cache = state.prayer_cache.lock().unwrap();
                        *cache = Some(CachedPrayers {
                            date: today.clone(),
                            data,
                        });
                    }
                }
            }

            // Read cached prayer data
            let prayer_data = {
                let cache = state.prayer_cache.lock().unwrap();
                cache.as_ref().map(|c| c.data.clone())
            };

            let Some(data) = prayer_data else { continue };

            let current_minutes = now.hour() as i32 * 60 + now.minute() as i32;

            let mut notified = state.notified_today.lock().unwrap();
            if notified.0 != today {
                notified.0 = today.clone();
                notified.1.clear();
            }

            let mut prayer_just_passed = false;

            for prayer in &data.prayers {
                let parts: Vec<&str> = prayer.time.split(':').collect();
                if parts.len() != 2 {
                    continue;
                }
                let h: i32 = parts[0].parse().unwrap_or(0);
                let m: i32 = parts[1].parse().unwrap_or(0);
                let prayer_minutes = h * 60 + m;
                let diff = prayer_minutes - current_minutes;

                // Advance notification (1-minute window before target)
                let adv_key = format!("{}_adv_{}", prayer.name, today);
                if diff <= config.notify_before_mins
                    && diff > config.notify_before_mins - 1
                    && !notified.1.contains(&adv_key)
                {
                    let _ = app
                        .notification()
                        .builder()
                        .title(&format!(
                            "{} in {} min",
                            prayer.name, config.notify_before_mins
                        ))
                        .body(&format!(
                            "{} prayer at {}. Prepare for salah.",
                            prayer.name, prayer.time
                        ))
                        .show();
                    play_sound_internal("Ping");
                    notified.1.insert(adv_key);
                }

                // Exact time notification (1-minute window at prayer time)
                let exact_key = format!("{}_exact_{}", prayer.name, today);
                if diff <= 0 && diff > -1 && !notified.1.contains(&exact_key) {
                    let _ = app
                        .notification()
                        .builder()
                        .title(&format!("{} Time", prayer.name))
                        .body(&format!(
                            "It's time for {} prayer. Allahu Akbar!",
                            prayer.name
                        ))
                        .show();
                    play_sound_internal("Glass");
                    notified.1.insert(exact_key);
                    prayer_just_passed = true;
                }
            }

            // Refresh cache and notify frontend when a prayer passes
            if prayer_just_passed {
                if let Ok(dir) = app.path().app_data_dir() {
                    if let Ok(new_data) = tauri::async_runtime::block_on(
                        fetch_prayer_times(&config, &dir),
                    ) {
                        let mut cache = state.prayer_cache.lock().unwrap();
                        *cache = Some(CachedPrayers {
                            date: today,
                            data: new_data,
                        });
                    }
                }
                let _ = app.emit("refresh-prayers", ());
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let config = load_config(app.handle());
            app.manage(AppState {
                config: Mutex::new(config),
                prayer_cache: Mutex::new(None),
                notified_today: Mutex::new((String::new(), HashSet::new())),
            });
            tray::create_tray(app)?;
            start_notification_checker(app.handle().clone());
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
