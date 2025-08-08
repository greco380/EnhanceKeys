use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use log::{info, error};
use tauri::Manager;
use rdev::{listen, Event, EventType, Key};

type HotkeyMap = Arc<Mutex<HashMap<String, String>>>;

static mut HOTKEYS: Option<HotkeyMap> = None;
static mut LISTENING: bool = false;

pub fn init_hotkeys() -> Result<()> {
    unsafe {
        HOTKEYS = Some(Arc::new(Mutex::new(HashMap::new())));
        start_listening()?;
    }
    Ok(())
}

unsafe fn start_listening() -> Result<()> {
    if LISTENING {
        return Ok(());
    }
    
    let hotkeys = HOTKEYS.as_ref().unwrap().clone();
    
    std::thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                handle_key_press(key, &hotkeys);
            }
        }) {
            error!("Failed to start hotkey listener: {}", e);
        }
    });
    
    LISTENING = true;
    info!("Hotkey listener started");
    Ok(())
}

fn handle_key_press(key: Key, hotkeys: &HotkeyMap) {
    // This is a simplified implementation
    // In a real implementation, you would track modifier keys (Ctrl, Shift, etc.)
    // and match against the registered hotkeys
    
    let key_str = match key {
        Key::KeyG => "G",
        Key::KeyR => "R",
        Key::KeyB => "B",
        Key::KeyF => "F",
        Key::KeyT => "T",
        Key::KeyC => "C",
        Key::KeyE => "E",
        Key::KeyO => "O",
        Key::KeyS => "S",
        Key::KeyD => "D",
        _ => return,
    };
    
    if let Ok(hotkeys_map) = hotkeys.lock() {
        for (snippet_id, hotkey) in hotkeys_map.iter() {
            if hotkey.contains(key_str) {
                info!("Hotkey triggered for snippet: {}", snippet_id);
                // Here you would trigger the text injection
                // This would typically involve sending a message to the main thread
                break;
            }
        }
    }
}

#[tauri::command]
pub async fn register_hotkey(snippet_id: String, hotkey: String) -> Result<(), String> {
    unsafe {
        if let Some(hotkeys) = &HOTKEYS {
            if let Ok(mut map) = hotkeys.lock() {
                map.insert(snippet_id, hotkey);
                info!("Registered hotkey: {} -> {}", snippet_id, hotkey);
                return Ok(());
            }
        }
    }
    Err("Failed to register hotkey".to_string())
}

#[tauri::command]
pub async fn unregister_hotkey(snippet_id: String) -> Result<(), String> {
    unsafe {
        if let Some(hotkeys) = &HOTKEYS {
            if let Ok(mut map) = hotkeys.lock() {
                map.remove(&snippet_id);
                info!("Unregistered hotkey for snippet: {}", snippet_id);
                return Ok(());
            }
        }
    }
    Err("Failed to unregister hotkey".to_string())
}

pub fn parse_hotkey(hotkey: &str) -> Result<Vec<Key>> {
    let mut keys = Vec::new();
    let parts: Vec<&str> = hotkey.split('+').collect();
    
    for part in parts {
        let key = match part.trim().to_uppercase().as_str() {
            "CTRL" => Key::ControlLeft,
            "SHIFT" => Key::ShiftLeft,
            "ALT" => Key::AltLeft,
            "G" => Key::KeyG,
            "R" => Key::KeyR,
            "B" => Key::KeyB,
            "F" => Key::KeyF,
            "T" => Key::KeyT,
            "C" => Key::KeyC,
            "E" => Key::KeyE,
            "O" => Key::KeyO,
            "S" => Key::KeyS,
            "D" => Key::KeyD,
            _ => return Err(anyhow::anyhow!("Unknown key: {}", part)),
        };
        keys.push(key);
    }
    
    Ok(keys)
} 