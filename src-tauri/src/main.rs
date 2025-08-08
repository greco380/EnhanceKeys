// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod hotkeys;
mod text_injection;
mod system_integration;

use tauri::Manager;
use log::{info, error};

fn main() {
    env_logger::init();
    info!("Starting EnhanceKeys application");

    tauri::Builder::default()
        .setup(|app| {
            info!("Setting up application");
            
            // Initialize configuration
            if let Err(e) = config::init_config() {
                error!("Failed to initialize configuration: {}", e);
            }

            // Initialize hotkeys
            if let Err(e) = hotkeys::init_hotkeys() {
                error!("Failed to initialize hotkeys: {}", e);
            }

            // Initialize system integration
            if let Err(e) = system_integration::init() {
                error!("Failed to initialize system integration: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            config::load_config,
            config::save_config,
            text_injection::inject_text,
            hotkeys::register_hotkey,
            hotkeys::unregister_hotkey,
            system_integration::get_active_window_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 