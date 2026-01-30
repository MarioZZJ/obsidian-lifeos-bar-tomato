use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerMode {
    Pomodoro,
    Stopwatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerPhase {
    Idle,
    Running,
    Paused,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerStatus {
    pub phase: TimerPhase,
    pub mode: TimerMode,
    pub elapsed_secs: u64,
    pub remaining_secs: Option<u64>,
    pub overtime_secs: u64,
    pub pomodoro_count: u32,
    pub current_task: Option<String>,
    pub current_project: Option<String>,
    pub current_project_path: Option<String>,
}

pub struct TimerState {
    pub phase: TimerPhase,
    pub mode: TimerMode,
    pub start_instant: Option<Instant>,
    pub start_timestamp_ms: Option<u64>,
    pub pause_elapsed: std::time::Duration,
    pub duration_secs: u64,
    pub pomodoro_count: u32,
    pub current_task: Option<String>,
    pub current_project: Option<String>,
    pub current_project_path: Option<String>,
    pub completion_notified: bool,
    // Config
    pub pomodoro_duration: u32,
    pub short_break_duration: u32,
    pub long_break_duration: u32,
    pub long_break_interval: u32,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            phase: TimerPhase::Idle,
            mode: TimerMode::Pomodoro,
            start_instant: None,
            start_timestamp_ms: None,
            pause_elapsed: std::time::Duration::ZERO,
            duration_secs: 25 * 60,
            pomodoro_count: 0,
            current_task: None,
            current_project: None,
            current_project_path: None,
            completion_notified: false,
            pomodoro_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            long_break_interval: 4,
        }
    }

    pub fn start_pomodoro(&mut self) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        self.mode = TimerMode::Pomodoro;
        self.phase = TimerPhase::Running;
        self.start_instant = Some(Instant::now());
        self.start_timestamp_ms = Some(now_ms);
        self.pause_elapsed = std::time::Duration::ZERO;
        self.duration_secs = (self.pomodoro_duration as u64) * 60;
        self.completion_notified = false;
    }

    pub fn start_stopwatch(&mut self) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        self.mode = TimerMode::Stopwatch;
        self.phase = TimerPhase::Running;
        self.start_instant = Some(Instant::now());
        self.start_timestamp_ms = Some(now_ms);
        self.pause_elapsed = std::time::Duration::ZERO;
        self.duration_secs = 0;
        self.completion_notified = false;
    }

    pub fn pause(&mut self) {
        if self.phase == TimerPhase::Running {
            if let Some(start) = self.start_instant {
                self.pause_elapsed += start.elapsed();
            }
            self.start_instant = None;
            self.phase = TimerPhase::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.phase == TimerPhase::Paused {
            self.start_instant = Some(Instant::now());
            self.phase = TimerPhase::Running;
        }
    }

    pub fn stop(&mut self) {
        self.phase = TimerPhase::Idle;
        self.start_instant = None;
        self.start_timestamp_ms = None;
        self.pause_elapsed = std::time::Duration::ZERO;
    }

    pub fn start_break(&mut self) {
        self.start_instant = Some(Instant::now());
        self.pause_elapsed = std::time::Duration::ZERO;
        self.completion_notified = false;

        if self.pomodoro_count > 0 && self.pomodoro_count % self.long_break_interval == 0 {
            self.phase = TimerPhase::LongBreak;
            self.duration_secs = (self.long_break_duration as u64) * 60;
        } else {
            self.phase = TimerPhase::ShortBreak;
            self.duration_secs = (self.short_break_duration as u64) * 60;
        }
    }

    pub fn skip_break(&mut self) {
        self.phase = TimerPhase::Idle;
        self.start_instant = None;
        self.pause_elapsed = std::time::Duration::ZERO;
    }

    pub fn elapsed(&self) -> std::time::Duration {
        let running_elapsed = self
            .start_instant
            .map(|s| s.elapsed())
            .unwrap_or(std::time::Duration::ZERO);
        self.pause_elapsed + running_elapsed
    }

    pub fn remaining_secs(&self) -> Option<u64> {
        if self.mode == TimerMode::Stopwatch && self.phase == TimerPhase::Running {
            return None;
        }
        if self.duration_secs == 0 {
            return None;
        }
        let elapsed = self.elapsed().as_secs();
        Some(self.duration_secs.saturating_sub(elapsed))
    }

    pub fn overtime_secs(&self) -> u64 {
        if self.mode == TimerMode::Pomodoro && self.phase == TimerPhase::Running {
            let elapsed = self.elapsed().as_secs();
            if elapsed > self.duration_secs {
                return elapsed - self.duration_secs;
            }
        }
        0
    }

    pub fn is_completed(&self) -> bool {
        match self.phase {
            TimerPhase::Running if self.mode == TimerMode::Pomodoro => {
                self.elapsed().as_secs() >= self.duration_secs
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                self.elapsed().as_secs() >= self.duration_secs
            }
            _ => false,
        }
    }

    pub fn status(&self) -> TimerStatus {
        TimerStatus {
            phase: self.phase,
            mode: self.mode,
            elapsed_secs: self.elapsed().as_secs(),
            remaining_secs: self.remaining_secs(),
            overtime_secs: self.overtime_secs(),
            pomodoro_count: self.pomodoro_count,
            current_task: self.current_task.clone(),
            current_project: self.current_project.clone(),
            current_project_path: self.current_project_path.clone(),
        }
    }

    pub fn tray_title(&self) -> String {
        match self.phase {
            TimerPhase::Idle => String::new(),
            TimerPhase::Running => {
                if self.mode == TimerMode::Pomodoro {
                    let overtime = self.overtime_secs();
                    if overtime > 0 {
                        let mins = overtime / 60;
                        let secs = overtime % 60;
                        format!(" +{:02}:{:02}", mins, secs)
                    } else if let Some(remaining) = self.remaining_secs() {
                        let mins = remaining / 60;
                        let secs = remaining % 60;
                        format!(" {:02}:{:02}", mins, secs)
                    } else {
                        String::new()
                    }
                } else {
                    let elapsed = self.elapsed().as_secs();
                    let mins = elapsed / 60;
                    let secs = elapsed % 60;
                    format!(" {:02}:{:02}", mins, secs)
                }
            }
            TimerPhase::Paused => {
                if self.mode == TimerMode::Pomodoro {
                    let overtime = self.overtime_secs();
                    if overtime > 0 {
                        let mins = overtime / 60;
                        let secs = overtime % 60;
                        format!(" ⏸ +{:02}:{:02}", mins, secs)
                    } else if let Some(remaining) = self.remaining_secs() {
                        let mins = remaining / 60;
                        let secs = remaining % 60;
                        format!(" ⏸ {:02}:{:02}", mins, secs)
                    } else {
                        " ⏸".to_string()
                    }
                } else {
                    let elapsed = self.elapsed().as_secs();
                    let mins = elapsed / 60;
                    let secs = elapsed % 60;
                    format!(" ⏸ {:02}:{:02}", mins, secs)
                }
            }
            TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                if let Some(remaining) = self.remaining_secs() {
                    let mins = remaining / 60;
                    let secs = remaining % 60;
                    format!(" ☕ {:02}:{:02}", mins, secs)
                } else {
                    " ☕".to_string()
                }
            }
        }
    }
}
