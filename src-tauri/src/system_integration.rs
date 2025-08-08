use anyhow::{Result, Context};
use log::{info, error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
    pub is_chat_window: bool,
}

pub fn init() -> Result<()> {
    info!("Initializing system integration");
    
    // Platform-specific initialization
    #[cfg(target_os = "windows")]
    {
        // Windows-specific initialization
        info!("Running on Windows");
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS-specific initialization
        info!("Running on macOS");
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux-specific initialization
        info!("Running on Linux");
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_active_window_info() -> Result<WindowInfo, String> {
    get_active_window_info_internal()
        .map_err(|e| {
            error!("Failed to get active window info: {}", e);
            e.to_string()
        })
}

fn get_active_window_info_internal() -> Result<WindowInfo> {
    // This is a simplified implementation
    // In a real implementation, you would use platform-specific APIs
    // to get the actual active window information
    
    let window_info = WindowInfo {
        title: "Unknown Window".to_string(),
        process_name: "unknown".to_string(),
        is_chat_window: false,
    };
    
    Ok(window_info)
}

pub fn is_chat_window(window_title: &str, process_name: &str) -> bool {
    let chat_indicators = [
        "chat",
        "message",
        "discord",
        "slack",
        "teams",
        "whatsapp",
        "telegram",
        "signal",
        "gpt",
        "claude",
        "bard",
        "copilot",
        "assistant",
    ];
    
    let title_lower = window_title.to_lowercase();
    let process_lower = process_name.to_lowercase();
    
    for indicator in &chat_indicators {
        if title_lower.contains(indicator) || process_lower.contains(indicator) {
            return true;
        }
    }
    
    false
}

pub fn get_system_info() -> Result<SystemInfo> {
    let info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: "1.0.0".to_string(),
    };
    
    Ok(info)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: String,
}

pub fn check_permissions() -> Result<Permissions> {
    // This would check if the app has necessary permissions
    // like accessibility permissions on macOS
    let permissions = Permissions {
        accessibility: true, // Simplified
        input_monitoring: true, // Simplified
    };
    
    Ok(permissions)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub accessibility: bool,
    pub input_monitoring: bool,
} 