use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroConfig {
    pub pomodoro_duration: u32,
    pub short_break_duration: u32,
    pub long_break_duration: u32,
    pub long_break_interval: u32,
    pub auto_start_break: bool,
    pub pomodoro_sound: bool,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            pomodoro_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            long_break_interval: 4,
            auto_start_break: false,
            pomodoro_sound: true,
        }
    }
}

pub fn read_lifeos_config(vault_path: &str) -> Result<PomodoroConfig, String> {
    let data_json_path = Path::new(vault_path)
        .join(".obsidian")
        .join("plugins")
        .join("lifeos-pro")
        .join("data.json");

    if !data_json_path.exists() {
        return Err("lifeos-pro plugin data.json not found".to_string());
    }

    let content = std::fs::read_to_string(&data_json_path)
        .map_err(|e| format!("Failed to read data.json: {}", e))?;

    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse data.json: {}", e))?;

    Ok(PomodoroConfig {
        pomodoro_duration: json
            .get("pomodoroDuration")
            .and_then(|v| v.as_u64())
            .unwrap_or(25) as u32,
        short_break_duration: json
            .get("shortBreakDuration")
            .and_then(|v| v.as_u64())
            .unwrap_or(5) as u32,
        long_break_duration: json
            .get("longBreakDuration")
            .and_then(|v| v.as_u64())
            .unwrap_or(15) as u32,
        long_break_interval: json
            .get("longBreakInterval")
            .and_then(|v| v.as_u64())
            .unwrap_or(4) as u32,
        auto_start_break: json
            .get("autoStartBreak")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        pomodoro_sound: json
            .get("pomodoroSound")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
    })
}

pub fn check_vault_valid(vault_path: &str) -> Result<bool, String> {
    let obsidian_dir = Path::new(vault_path).join(".obsidian");
    if !obsidian_dir.exists() {
        return Ok(false);
    }
    let lifeos_dir = obsidian_dir.join("plugins").join("lifeos-pro");
    Ok(lifeos_dir.exists())
}
