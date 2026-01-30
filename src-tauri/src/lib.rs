mod app_config;
mod app_state;
mod autostart;
mod commands;
mod platform;
mod timer;
mod vault;

use app_state::AppState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};
#[cfg(not(target_os = "windows"))]
use tauri::WindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let device_hash = platform::get_device_hash();

    // Load saved config
    let saved_config = app_config::load_config();

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize app state with saved vault path
            let app_state = AppState::new(device_hash.clone());
            if let Some(ref vault_path) = saved_config.vault_path {
                *app_state.vault_path.lock().unwrap() = Some(vault_path.clone());

                // Try to load config from vault
                if let Ok(cfg) = vault::read_lifeos_config(vault_path) {
                    *app_state.config.lock().unwrap() = cfg.clone();
                    let mut timer = app_state.timer.lock().unwrap();
                    timer.pomodoro_duration = cfg.pomodoro_duration;
                    timer.short_break_duration = cfg.short_break_duration;
                    timer.long_break_duration = cfg.long_break_duration;
                    timer.long_break_interval = cfg.long_break_interval;
                }
            }

            app.manage(app_state);

            // Create tray menu
            let quit_item = MenuItemBuilder::with_id("quit", "退出 Bar Tomato").build(app)?;
            let tray_menu = MenuBuilder::new(app).item(&quit_item).build()?;

            // Create tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .tooltip("Bar Tomato")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id().as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // Let positioner track tray position
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            let is_minimized = window.is_minimized().unwrap_or(false);

                            if is_visible && !is_minimized {
                                let _ = window.hide();
                            } else {
                                // Restore from minimized state if needed
                                if is_minimized {
                                    let _ = window.unminimize();
                                }

                                // Position window near tray icon on macOS
                                #[cfg(target_os = "macos")]
                                {
                                    use tauri::PhysicalPosition;

                                    if let Ok(Some(rect)) = tray.rect() {
                                        if let Ok(monitor) = window.current_monitor() {
                                            if let Some(monitor) = monitor {
                                                let screen_size = monitor.size();
                                                let window_width = 320.0;
                                                let window_height = 480.0;

                                                let (icon_x, icon_y, icon_width, icon_height) = match (rect.position, rect.size) {
                                                    (tauri::Position::Physical(pos), tauri::Size::Physical(size)) => {
                                                        (pos.x as f64, pos.y as f64, size.width as f64, size.height as f64)
                                                    }
                                                    (tauri::Position::Logical(pos), tauri::Size::Logical(size)) => {
                                                        (pos.x, pos.y, size.width, size.height)
                                                    }
                                                    _ => (0.0, 0.0, 0.0, 0.0),
                                                };

                                                let x = (icon_x + icon_width / 2.0 - window_width / 2.0).max(0.0);
                                                let y = (icon_y + icon_height).min(screen_size.height as f64 - window_height);

                                                let _ = window.set_position(PhysicalPosition::new(x as i32, y as i32));
                                            }
                                        }
                                    }
                                }

                                // Position window at bottom-right of work area on Windows
                                #[cfg(target_os = "windows")]
                                {
                                    use tauri::PhysicalPosition;

                                    if let Ok(Some(monitor)) = window.current_monitor() {
                                        let monitor_pos = monitor.position();
                                        let monitor_size = monitor.size();
                                        let scale = monitor.scale_factor();

                                        // Work area approximation: leave 48px (logical) for taskbar
                                        let taskbar_height = (48.0 * scale) as i32;

                                        let win_width = (400.0 * scale) as i32;
                                        let win_height = (600.0 * scale) as i32;
                                        let margin = (12.0 * scale) as i32;

                                        let x = monitor_pos.x + monitor_size.width as i32 - win_width - margin;
                                        let y = monitor_pos.y + monitor_size.height as i32 - taskbar_height - win_height - margin;

                                        let _ = window.set_position(PhysicalPosition::new(x, y));
                                    }
                                }

                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Apply macOS vibrancy effect
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

                let window = app.get_webview_window("main").unwrap();

                // Set webview background to transparent
                let _ = window.set_background_color(Some(tauri::window::Color(0, 0, 0, 0)));

                apply_vibrancy(&window, NSVisualEffectMaterial::Popover, None, None)
                    .expect("Failed to apply vibrancy effect");
            }

            // Apply Windows acrylic effect and initial positioning
            #[cfg(target_os = "windows")]
            {
                use tauri::PhysicalPosition;
                use window_vibrancy::apply_acrylic;

                let window = app.get_webview_window("main").unwrap();
                let _ = window.set_background_color(Some(tauri::window::Color(0, 0, 0, 0)));
                let _ = apply_acrylic(&window, Some((255, 255, 255, 40)));

                // Position window at bottom-right of screen, above taskbar
                if let Ok(Some(monitor)) = window.current_monitor() {
                    let monitor_pos = monitor.position();
                    let monitor_size = monitor.size();
                    let scale = monitor.scale_factor();

                    let taskbar_height = (48.0 * scale) as i32;
                    let win_width = (400.0 * scale) as i32;
                    let win_height = (600.0 * scale) as i32;
                    let margin = (12.0 * scale) as i32;

                    let x = monitor_pos.x + monitor_size.width as i32 - win_width - margin;
                    let y = monitor_pos.y + monitor_size.height as i32 - taskbar_height - win_height - margin;

                    let _ = window.set_position(PhysicalPosition::new(x, y));
                }
            }

            // Hide dock icon on macOS
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Timer tick loop
            let app_handle = app.handle().clone();
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = running.clone();

            std::thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_secs(1));

                    let state = app_handle.state::<AppState>();
                    let mut timer = state.timer.lock().unwrap();

                    // Check if timer completed (only notify once)
                    if timer.is_completed() && !timer.completion_notified {
                        timer.completion_notified = true;
                        match timer.phase {
                            timer::TimerPhase::Running => {
                                if timer.mode == timer::TimerMode::Pomodoro {
                                    let _ =
                                        app_handle.emit("pomodoro-complete", timer.pomodoro_count);
                                }
                            }
                            timer::TimerPhase::ShortBreak | timer::TimerPhase::LongBreak => {
                                let _ = app_handle.emit("break-complete", ());
                            }
                            _ => {}
                        }
                    }

                    // Update tray title
                    let title = timer.tray_title();

                    // Gather progress bar info while we have the lock
                    let phase = timer.phase;
                    let mode = timer.mode;
                    let elapsed_secs = timer.elapsed().as_secs();
                    let duration_secs = timer.duration_secs;

                    drop(timer); // Release lock before updating tray/window

                    if let Some(tray) = app_handle.tray_by_id("main") {
                        let _ = tray.set_title(Some(&title));

                        // Update tooltip (visible on hover in Windows)
                        let tooltip = if title.is_empty() {
                            "Bar Tomato".to_string()
                        } else {
                            format!("Bar Tomato{}", title)
                        };
                        let _ = tray.set_tooltip(Some(&tooltip));
                    }

                    // Update window title and progress bar (visible in Windows taskbar)
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let window_title = if title.is_empty() {
                            "Bar Tomato".to_string()
                        } else {
                            format!("Bar Tomato{}", title)
                        };
                        let _ = window.set_title(&window_title);

                        // Update taskbar progress bar
                        use tauri::window::{ProgressBarState, ProgressBarStatus};
                        match phase {
                            timer::TimerPhase::Running => {
                                if mode == timer::TimerMode::Pomodoro && duration_secs > 0 {
                                    let progress = ((elapsed_secs as f64 / duration_secs as f64) * 100.0).min(100.0) as u64;
                                    let _ = window.set_progress_bar(ProgressBarState {
                                        status: Some(ProgressBarStatus::Normal),
                                        progress: Some(progress),
                                    });
                                } else if mode == timer::TimerMode::Stopwatch {
                                    let _ = window.set_progress_bar(ProgressBarState {
                                        status: Some(ProgressBarStatus::Indeterminate),
                                        progress: None,
                                    });
                                }
                            }
                            timer::TimerPhase::Paused => {
                                if duration_secs > 0 {
                                    let progress = ((elapsed_secs as f64 / duration_secs as f64) * 100.0).min(100.0) as u64;
                                    let _ = window.set_progress_bar(ProgressBarState {
                                        status: Some(ProgressBarStatus::Paused),
                                        progress: Some(progress),
                                    });
                                } else {
                                    let _ = window.set_progress_bar(ProgressBarState {
                                        status: Some(ProgressBarStatus::Paused),
                                        progress: None,
                                    });
                                }
                            }
                            timer::TimerPhase::ShortBreak | timer::TimerPhase::LongBreak => {
                                if duration_secs > 0 {
                                    let progress = ((elapsed_secs as f64 / duration_secs as f64) * 100.0).min(100.0) as u64;
                                    let _ = window.set_progress_bar(ProgressBarState {
                                        status: Some(ProgressBarStatus::Normal),
                                        progress: Some(progress),
                                    });
                                }
                            }
                            timer::TimerPhase::Idle => {
                                let _ = window.set_progress_bar(ProgressBarState {
                                    status: Some(ProgressBarStatus::None),
                                    progress: None,
                                });
                            }
                        }
                    }

                    // Emit tick event for frontend
                    let _ = app_handle.emit("timer-tick", ());
                }
            });

            // Handle window losing focus
            // macOS: hide window (popover behavior)
            // Windows: do nothing, behave like a normal window
            #[cfg(not(target_os = "windows"))]
            {
                let main_window = app.get_webview_window("main").unwrap();
                let main_window_clone = main_window.clone();
                let app_handle_for_focus = app.handle().clone();
                main_window.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        let state = app_handle_for_focus.state::<AppState>();
                        if !state.dialog_open.load(Ordering::Relaxed) {
                            let _ = main_window_clone.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_timer_status,
            commands::start_pomodoro,
            commands::start_stopwatch,
            commands::pause_timer,
            commands::resume_timer,
            commands::stop_timer,
            commands::complete_pomodoro,
            commands::skip_break,
            commands::complete_break,
            commands::set_vault_path,
            commands::get_vault_path,
            commands::get_config,
            commands::scan_projects,
            commands::scan_tasks,
            commands::get_tray_title,
            commands::get_today_stats,
            commands::set_autostart,
            commands::get_autostart,
            commands::set_dialog_open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
