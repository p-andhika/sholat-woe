use chrono::{Local, NaiveTime, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ─── API Response Types ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarResponse {
    pub code: i32,
    pub data: HashMap<String, Vec<DayData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayData {
    pub timings: Timings,
    pub date: DateInfo,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timings {
    #[serde(rename = "Fajr")]
    pub fajr: String,
    #[serde(rename = "Sunrise")]
    pub sunrise: String,
    #[serde(rename = "Dhuhr")]
    pub dhuhr: String,
    #[serde(rename = "Asr")]
    pub asr: String,
    #[serde(rename = "Sunset")]
    pub sunset: String,
    #[serde(rename = "Maghrib")]
    pub maghrib: String,
    #[serde(rename = "Isha")]
    pub isha: String,
    #[serde(rename = "Imsak")]
    pub imsak: String,
    #[serde(rename = "Midnight")]
    pub midnight: String,
    #[serde(rename = "Firstthird")]
    pub firstthird: String,
    #[serde(rename = "Lastthird")]
    pub lastthird: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateInfo {
    pub readable: String,
    pub hijri: HijriDate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HijriDate {
    pub date: String,
    pub day: String,
    pub month: HijriMonth,
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HijriMonth {
    pub number: i32,
    pub en: String,
    pub ar: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub method: MethodInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MethodInfo {
    pub id: i32,
    pub name: String,
}

// ─── Frontend-facing types ──────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
pub struct PrayerTime {
    pub name: String,
    pub time: String,
    pub passed: bool,
    pub is_next: bool,
    pub icon: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrayerTimesResult {
    pub prayers: Vec<PrayerTime>,
    pub sunrise: String,
    pub sunset: String,
    pub imsak: String,
    pub midnight: String,
    pub firstthird: String,
    pub lastthird: String,
    pub hijri_date: String,
    pub gregorian_date: String,
    pub next_prayer: String,
    pub next_prayer_time: String,
    pub time_remaining: String,
    pub method_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrayerConfig {
    pub city: String,
    pub country: String,
    pub method: i32,
    pub school: i32,
    pub notify_before_mins: i32,
}

impl Default for PrayerConfig {
    fn default() -> Self {
        Self {
            city: "Jakarta".to_string(),
            country: "ID".to_string(),
            method: 20,
            school: 0,
            notify_before_mins: 5,
        }
    }
}

fn parse_time(time_str: &str) -> Option<NaiveTime> {
    let clean = time_str.split(' ').next().unwrap_or(time_str);
    NaiveTime::parse_from_str(clean, "%H:%M").ok()
}

fn get_icon(name: &str) -> &str {
    match name {
        "Imsak" => "moon-star",
        "Fajr" => "sunrise",
        "Dhuhr" => "sun",
        "Asr" => "cloud-sun",
        "Maghrib" => "sunset",
        "Isha" => "moon",
        _ => "clock",
    }
}

// ─── Cache Types ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CachedCalendar {
    year: i32,
    config_hash: String,
    data: HashMap<String, Vec<DayData>>,
}

/// Creates a cache key from config fields that affect prayer time calculation.
/// Note: notify_before_mins is excluded as it doesn't affect the API response.
fn config_hash(config: &PrayerConfig) -> String {
    format!("{}-{}-{}-{}", config.city, config.country, config.method, config.school)
}

fn cache_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("prayer_cache.json")
}

fn load_cache(app_data_dir: &PathBuf) -> Option<CachedCalendar> {
    let path = cache_path(app_data_dir);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

fn save_cache(app_data_dir: &PathBuf, cache: &CachedCalendar) {
    let path = cache_path(app_data_dir);
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create cache directory: {}", e);
            return;
        }
    }
    match serde_json::to_string_pretty(cache) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                eprintln!("Failed to write cache file: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize cache: {}", e),
    }
}

async fn fetch_and_cache_calendar(
    year: i32,
    config: &PrayerConfig,
    app_data_dir: &PathBuf,
    config_hash: &str,
) -> Result<HashMap<String, Vec<DayData>>, String> {
    let url = format!(
        "https://api.aladhan.com/v1/calendarByCity/{}?city={}&country={}&method={}&school={}",
        year, config.city, config.country, config.method, config.school
    );

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let calendar: CalendarResponse = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    // Save to cache
    let cached = CachedCalendar {
        year,
        config_hash: config_hash.to_string(),
        data: calendar.data.clone(),
    };
    save_cache(app_data_dir, &cached);

    Ok(calendar.data)
}

pub async fn fetch_prayer_times(config: &PrayerConfig, app_data_dir: &PathBuf) -> Result<PrayerTimesResult, String> {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let current_hash = config_hash(config);

    // Try to load from cache
    let calendar_data = if let Some(cached) = load_cache(app_data_dir) {
        if cached.year == year && cached.config_hash == current_hash {
            // Cache hit - use cached data
            cached.data
        } else {
            // Cache miss - fetch new data
            fetch_and_cache_calendar(year, config, app_data_dir, &current_hash).await?
        }
    } else {
        // No cache - fetch new data
        fetch_and_cache_calendar(year, config, app_data_dir, &current_hash).await?
    };

    let month_key = month.to_string();
    let month_data = calendar_data.get(&month_key)
        .ok_or_else(|| format!("Month {} not found in calendar", month))?;

    let today_data = month_data.get((day - 1) as usize)
        .ok_or_else(|| format!("Day {} not found in month {}", day, month))?;

    let timings = &today_data.timings;
    let now_time = now.time();

    let prayer_names = vec!["Imsak", "Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"];
    let prayer_times_raw = vec![
        &timings.imsak,
        &timings.fajr,
        &timings.dhuhr,
        &timings.asr,
        &timings.maghrib,
        &timings.isha,
    ];

    let mut prayers: Vec<PrayerTime> = Vec::new();
    let mut next_found = false;
    let mut next_prayer_name = String::new();
    let mut next_prayer_time_str = String::new();
    let mut time_remaining = String::new();

    for (i, name) in prayer_names.iter().enumerate() {
        let time_str = prayer_times_raw[i].split(' ').next().unwrap_or("--:--");
        let parsed = parse_time(prayer_times_raw[i]);
        let passed = parsed.map(|t| now_time > t).unwrap_or(false);

        // Only consider Fajr, Dhuhr, Asr, Maghrib, Isha as "next prayer" (skip Imsak)
        let is_next = if !next_found && !passed && *name != "Imsak" {
            next_found = true;
            next_prayer_name = name.to_string();
            next_prayer_time_str = time_str.to_string();

            if let Some(pt) = parsed {
                let diff = pt - now_time;
                let total_mins = diff.num_minutes();
                let hours = total_mins / 60;
                let mins = total_mins % 60;
                time_remaining = if hours > 0 {
                    format!("{}h {}m", hours, mins)
                } else {
                    format!("{}m", mins)
                };
            }
            true
        } else {
            false
        };

        prayers.push(PrayerTime {
            name: name.to_string(),
            time: time_str.to_string(),
            passed,
            is_next,
            icon: get_icon(name).to_string(),
        });
    }

    if !next_found {
        next_prayer_name = "Fajr".to_string();
        next_prayer_time_str = timings
            .fajr
            .split(' ')
            .next()
            .unwrap_or("--:--")
            .to_string();
        time_remaining = "tomorrow".to_string();

        // Mark Fajr as next prayer
        if let Some(fajr) = prayers.iter_mut().find(|p| p.name == "Fajr") {
            fajr.is_next = true;
        }
    }

    let hijri = &today_data.date.hijri;
    let month_name = hijri.month.en.replace("Ramaḍān", "Ramadhan");
    let hijri_date = format!("{} {} {}", hijri.day, month_name, hijri.year);

    Ok(PrayerTimesResult {
        prayers,
        sunrise: timings.sunrise.split(' ').next().unwrap_or("--:--").to_string(),
        sunset: timings.sunset.split(' ').next().unwrap_or("--:--").to_string(),
        imsak: timings.imsak.split(' ').next().unwrap_or("--:--").to_string(),
        midnight: timings.midnight.split(' ').next().unwrap_or("--:--").to_string(),
        firstthird: timings.firstthird.split(' ').next().unwrap_or("--:--").to_string(),
        lastthird: timings.lastthird.split(' ').next().unwrap_or("--:--").to_string(),
        hijri_date,
        gregorian_date: today_data.date.readable.clone(),
        next_prayer: next_prayer_name,
        next_prayer_time: next_prayer_time_str,
        time_remaining,
        method_name: today_data.meta.method.name.clone(),
    })
}
