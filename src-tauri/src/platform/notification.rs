use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

pub fn send_pomodoro_complete<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    app.notification()
        .builder()
        .title("🍅 番茄钟完成")
        .body("休息一下吧！")
        .show()
        .map_err(|e| format!("Failed to send notification: {}", e))
}

pub fn send_break_complete<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    app.notification()
        .builder()
        .title("☕ 休息结束")
        .body("准备开始下一个番茄钟！")
        .show()
        .map_err(|e| format!("Failed to send notification: {}", e))
}

pub fn send_stopwatch_stopped<R: Runtime>(app: &AppHandle<R>, duration_mins: u32) -> Result<(), String> {
    app.notification()
        .builder()
        .title("⏱ 计时完成")
        .body(&format!("本次计时 {} 分钟", duration_mins))
        .show()
        .map_err(|e| format!("Failed to send notification: {}", e))
}
