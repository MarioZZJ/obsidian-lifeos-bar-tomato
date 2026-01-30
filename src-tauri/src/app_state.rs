use crate::timer::TimerState;
use crate::vault::PomodoroConfig;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    pub timer: Mutex<TimerState>,
    pub vault_path: Mutex<Option<String>>,
    pub device_hash: String,
    pub config: Mutex<PomodoroConfig>,
    pub dialog_open: AtomicBool,
}

impl AppState {
    pub fn new(device_hash: String) -> Self {
        Self {
            timer: Mutex::new(TimerState::new()),
            vault_path: Mutex::new(None),
            device_hash,
            config: Mutex::new(PomodoroConfig::default()),
            dialog_open: AtomicBool::new(false),
        }
    }
}
