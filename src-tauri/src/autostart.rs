use tauri::AppHandle;

pub fn enable_autostart(_app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use std::path::Path;

        // Get the executable path
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        // Determine if it's an app bundle or standalone binary
        let app_bundle_path = if exe_path.ends_with(".app/Contents/MacOS/bar-tomato") {
            // It's inside an app bundle - get the .app bundle path
            exe_path
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
                .ok_or_else(|| "Failed to get app bundle path".to_string())?
        } else {
            // It's a standalone binary
            &exe_path
        };

        let is_app_bundle = app_bundle_path.extension().map_or(false, |ext| ext == "app");

        // Create LaunchAgents directory
        let agents_dir = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join("Library/LaunchAgents");

        fs::create_dir_all(&agents_dir)
            .map_err(|e| format!("Failed to create LaunchAgents directory: {}", e))?;

        // Create plist file
        let plist_path = agents_dir.join("com.mariozzj.bar-tomato.plist");

        let program_arguments = if is_app_bundle {
            format!(
                "<key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>{}</string>
    </array>",
                "/usr/bin/open",
                app_bundle_path.to_str().ok_or_else(|| "Invalid app path".to_string())?
            )
        } else {
            format!(
                "<key>Program</key>
    <string>{}</string>",
                app_bundle_path.to_str().ok_or_else(|| "Invalid app path".to_string())?
            )
        };

        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.mariozzj.bar-tomato</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Background</string>
    {}
</dict>
</plist>"#,
            program_arguments
        );

        fs::write(&plist_path, plist_content)
            .map_err(|e| format!("Failed to write plist file: {}", e))?;

        // Load the plist using launchctl
        let status = std::process::Command::new("launchctl")
            .arg("load")
            .arg(plist_path.to_str().ok_or_else(|| "Invalid plist path".to_string())?)
            .status()
            .map_err(|e| format!("Failed to load plist: {}", e))?;

        if !status.success() {
            return Err("Failed to load plist with launchctl".to_string());
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
        let (key, _) = hkcu
            .create_subkey(path)
            .map_err(|e| format!("Failed to open registry: {}", e))?;

        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?
            .to_str()
            .ok_or_else(|| "Invalid executable path".to_string())?
            .to_string();

        key.set_value("bar-tomato", &exe_path)
            .map_err(|e| format!("Failed to set registry value: {}", e))?;

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("Autostart not supported on this platform".to_string())
    }
}

pub fn disable_autostart(_app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::fs;

        // Remove the plist file
        let plist_path = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join("Library/LaunchAgents/com.mariozzj.bar-tomato.plist");

        // Unload the plist first
        let _ = std::process::Command::new("launchctl")
            .arg("unload")
            .arg(plist_path.to_str().ok_or_else(|| "Invalid plist path".to_string())?)
            .status();

        // Then remove the file
        if plist_path.exists() {
            fs::remove_file(&plist_path)
                .map_err(|e| format!("Failed to remove plist file: {}", e))?;
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
        let key = hkcu
            .open_subkey_with_flags(path, KEY_SET_VALUE)
            .map_err(|e| format!("Failed to open registry: {}", e))?;

        key.delete_value("bar-tomato")
            .map_err(|e| format!("Failed to delete registry value: {}", e))?;

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("Autostart not supported on this platform".to_string())
    }
}

#[allow(dead_code)]
pub fn is_autostart_enabled() -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        let plist_path = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join("Library/LaunchAgents/com.mariozzj.bar-tomato.plist");

        Ok(plist_path.exists())
    }

    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";

        match hkcu.open_subkey(path) {
            Ok(key) => {
                let exists = key.get_value::<String, _>("bar-tomato").is_ok();
                Ok(exists)
            }
            Err(_) => Ok(false),
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(false)
    }
}
