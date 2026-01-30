use crate::app_config;
use crate::app_state::AppState;
use crate::autostart;
use crate::platform;
use crate::timer::{TimerMode, TimerPhase, TimerStatus};
use crate::vault::{
    self, check_pomodoro_habit, update_project_time, PomodoroConfig, PomodoroRecord, Project,
    VaultTask,
};
use chrono::Local;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, State};
use uuid::Uuid;

#[tauri::command]
pub fn get_timer_status(state: State<AppState>) -> TimerStatus {
    state.timer.lock().unwrap().status()
}

#[tauri::command]
pub fn start_pomodoro(
    state: State<AppState>,
    task: Option<String>,
    project: Option<String>,
    project_path: Option<String>,
) {
    let mut timer = state.timer.lock().unwrap();
    timer.current_task = task;
    timer.current_project = project;
    timer.current_project_path = project_path;
    timer.start_pomodoro();
}

#[tauri::command]
pub fn start_stopwatch(
    state: State<AppState>,
    task: Option<String>,
    project: Option<String>,
    project_path: Option<String>,
) {
    let mut timer = state.timer.lock().unwrap();
    timer.current_task = task;
    timer.current_project = project;
    timer.current_project_path = project_path;
    timer.start_stopwatch();
}

#[tauri::command]
pub fn pause_timer(state: State<AppState>) {
    state.timer.lock().unwrap().pause();
}

#[tauri::command]
pub fn resume_timer(state: State<AppState>) {
    state.timer.lock().unwrap().resume();
}

#[tauri::command]
pub fn stop_timer(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    let mut timer = state.timer.lock().unwrap();
    let elapsed_mins = (timer.elapsed().as_secs() / 60) as u32;

    // Only record if there's meaningful elapsed time
    if elapsed_mins > 0 || timer.mode == TimerMode::Pomodoro {
        let vault_path = state.vault_path.lock().unwrap().clone();
        if let Some(ref vp) = vault_path {
            let date = Local::now().format("%Y-%m-%d").to_string();
            let record = PomodoroRecord {
                id: Uuid::new_v4().to_string(),
                date: date.clone(),
                start_time: timer.start_timestamp_ms.unwrap_or(0),
                end_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                duration: elapsed_mins,
                mode: match timer.mode {
                    TimerMode::Pomodoro => "pomodoro".to_string(),
                    TimerMode::Stopwatch => "stopwatch".to_string(),
                },
                status: "completed".to_string(),
                project_path: timer.current_project_path.clone(),
                task_text: timer.current_task.clone(),
                pomodoro_index: if timer.mode == TimerMode::Pomodoro {
                    Some(timer.pomodoro_count + 1)
                } else {
                    None
                },
            };

            // Write record
            let records_path = vault::get_records_file_path(vp, &state.device_hash);
            vault::append_record(&records_path, record)?;

            // Update daily note project time
            if let (Some(ref pp), Some(ref pn)) =
                (&timer.current_project_path, &timer.current_project)
            {
                let _ = update_project_time(vp, &date, pp, pn, elapsed_mins);
            }

            // Check pomodoro habit
            let _ = check_pomodoro_habit(vp, &date);
        }

        // Send notification for stopwatch
        if timer.mode == TimerMode::Stopwatch {
            let _ = platform::send_stopwatch_stopped(&app, elapsed_mins);
        }
    }

    timer.stop();
    Ok(())
}

#[tauri::command]
pub fn complete_pomodoro(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    let mut timer = state.timer.lock().unwrap();

    if timer.phase != TimerPhase::Running || timer.mode != TimerMode::Pomodoro {
        return Err("No pomodoro running".to_string());
    }

    let vault_path = state.vault_path.lock().unwrap().clone();
    let date = Local::now().format("%Y-%m-%d").to_string();

    timer.pomodoro_count += 1;
    let pomodoro_index = timer.pomodoro_count;

    // Use actual elapsed minutes (includes overtime)
    let actual_duration_mins = (timer.elapsed().as_secs() / 60).max(1) as u32;

    // Create record
    if let Some(ref vp) = vault_path {
        let record = PomodoroRecord {
            id: Uuid::new_v4().to_string(),
            date: date.clone(),
            start_time: timer.start_timestamp_ms.unwrap_or(0),
            end_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            duration: actual_duration_mins,
            mode: "pomodoro".to_string(),
            status: "completed".to_string(),
            project_path: timer.current_project_path.clone(),
            task_text: timer.current_task.clone(),
            pomodoro_index: Some(pomodoro_index),
        };

        // Write record
        let records_path = vault::get_records_file_path(vp, &state.device_hash);
        vault::append_record(&records_path, record)?;

        // Update daily note project time
        if let (Some(ref pp), Some(ref pn)) = (&timer.current_project_path, &timer.current_project)
        {
            let _ = update_project_time(vp, &date, pp, pn, actual_duration_mins);
        }

        // Check pomodoro habit
        let _ = check_pomodoro_habit(vp, &date);
    }

    // Send notification
    let _ = platform::send_pomodoro_complete(&app);

    // Start break
    timer.start_break();

    Ok(())
}

#[tauri::command]
pub fn skip_break(state: State<AppState>) {
    state.timer.lock().unwrap().skip_break();
}

#[tauri::command]
pub fn complete_break(app: AppHandle, state: State<AppState>) {
    let mut timer = state.timer.lock().unwrap();
    if timer.phase == TimerPhase::ShortBreak || timer.phase == TimerPhase::LongBreak {
        let _ = platform::send_break_complete(&app);
        timer.skip_break();
    }
}

#[tauri::command]
pub fn set_vault_path(state: State<AppState>, path: String) -> Result<PomodoroConfig, String> {
    if !vault::check_vault_valid(&path)? {
        return Err("Invalid vault or lifeos-pro plugin not found".to_string());
    }

    let config = vault::read_lifeos_config(&path)?;

    // Update timer config
    {
        let mut timer = state.timer.lock().unwrap();
        timer.pomodoro_duration = config.pomodoro_duration;
        timer.short_break_duration = config.short_break_duration;
        timer.long_break_duration = config.long_break_duration;
        timer.long_break_interval = config.long_break_interval;
    }

    // Store config
    *state.config.lock().unwrap() = config.clone();
    *state.vault_path.lock().unwrap() = Some(path.clone());

    // Save to persistent config
    let mut app_config = app_config::load_config();
    app_config.vault_path = Some(path);
    app_config::save_config(&app_config)?;

    Ok(config)
}

#[tauri::command]
pub fn get_vault_path(state: State<AppState>) -> Option<String> {
    state.vault_path.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_config(state: State<AppState>) -> PomodoroConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn scan_projects(state: State<AppState>) -> Result<Vec<Project>, String> {
    let vault_path = state
        .vault_path
        .lock()
        .unwrap()
        .clone()
        .ok_or("Vault not configured")?;
    vault::scan_projects(&vault_path)
}

#[tauri::command]
pub fn scan_tasks(state: State<AppState>) -> Result<Vec<VaultTask>, String> {
    let vault_path = state
        .vault_path
        .lock()
        .unwrap()
        .clone()
        .ok_or("Vault not configured")?;
    vault::scan_tasks(&vault_path)
}

#[tauri::command]
pub fn get_tray_title(state: State<AppState>) -> String {
    state.timer.lock().unwrap().tray_title()
}

#[tauri::command]
pub fn get_today_stats(state: State<AppState>) -> Result<TodayStats, String> {
    let vault_path = state
        .vault_path
        .lock()
        .unwrap()
        .clone()
        .ok_or("Vault not configured")?;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let records_path = vault::get_records_file_path(&vault_path, &state.device_hash);
    let records = vault::read_records(&records_path)?;

    let today_records: Vec<_> = records.records.iter().filter(|r| r.date == date).collect();

    let total_minutes: u32 = today_records.iter().map(|r| r.duration).sum();
    let pomodoro_count = today_records
        .iter()
        .filter(|r| r.mode == "pomodoro")
        .count() as u32;

    Ok(TodayStats {
        total_minutes,
        pomodoro_count,
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayStats {
    pub total_minutes: u32,
    pub pomodoro_count: u32,
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        autostart::enable_autostart(&app)?;
    } else {
        autostart::disable_autostart(&app)?;
    }

    // Update config
    let mut config = app_config::load_config();
    config.autostart = enabled;
    app_config::save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub fn get_autostart() -> Result<bool, String> {
    let config = app_config::load_config();
    Ok(config.autostart)
}

#[tauri::command]
pub fn set_dialog_open(state: State<AppState>, open: bool) {
    state.dialog_open.store(open, Ordering::Relaxed);
}
