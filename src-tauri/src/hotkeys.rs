use anyhow::Result;
use log::{info, error};
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}, GlobalHotKeyEvent};
use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;
use crate::text_injection::inject_text_internal;

static HOTKEY_MANAGER: LazyLock<Mutex<Option<GlobalHotKeyManager>>> = LazyLock::new(|| Mutex::new(None));
static REGISTERED_HOTKEYS: LazyLock<Mutex<HashMap<String, HotKey>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn init_hotkeys() -> Result<()> {
    info!("Initializing global hotkey system");
    
    let manager = GlobalHotKeyManager::new()?;
    
    // Register default Alt+Shift+1-9 hotkeys
    let hotkeys = [
        ("1", Code::Digit1),
        ("2", Code::Digit2),
        ("3", Code::Digit3),
        ("4", Code::Digit4),
        ("5", Code::Digit5),
        ("6", Code::Digit6),
        ("7", Code::Digit7),
        ("8", Code::Digit8),
        ("9", Code::Digit9),
    ];
    
    let mut registered = REGISTERED_HOTKEYS.lock().unwrap();
    
    for (key, code) in hotkeys {
        let hotkey = HotKey::new(Some(Modifiers::ALT | Modifiers::SHIFT), code);
        manager.register(hotkey)?;
        registered.insert(key.to_string(), hotkey);
        info!("Registered hotkey Alt+Shift+{} for snippet", key);
    }
    
    *HOTKEY_MANAGER.lock().unwrap() = Some(manager);
    
    // Start hotkey event listener in background thread
    std::thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(event) = receiver.try_recv() {
                if let Some(key) = find_key_for_hotkey(event.id()) {
                    handle_hotkey_event(&key);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    
    info!("Global hotkey system initialized successfully");
    Ok(())
}

fn find_key_for_hotkey(hotkey_id: u32) -> Option<String> {
    let registered = REGISTERED_HOTKEYS.lock().unwrap();
    for (key, hotkey) in registered.iter() {
        if hotkey.id() == hotkey_id {
            return Some(key.clone());
        }
    }
    None
}

fn handle_hotkey_event(key: &str) {
    println!("🔥 Hotkey Alt+Shift+{} triggered!", key);
    if let Some(text) = get_snippet_text(key) {
        println!("📝 Injecting: {}...", &text[..60.min(text.len())]);
        if let Err(e) = inject_text_internal(text) {
            println!("⚠️  Text injection failed: {}", e);
            println!("💡 This is expected in WSL - would work in native environment");
        } else {
            println!("✅ Text injected successfully!");
        }
    }
}

pub fn get_snippet_text(key: &str) -> Option<&'static str> {
    match key {
        "1" => Some("Ask the user questions in relation to supplementing the context you are given in the prompt. Ask the user questions until you are 95% sure you have the proper context in order to understand the problem and provide an accurate implementation/demo/mvp."),
        "2" => Some("If there is a situation where you encounter a problem you do not know how to fix/implement, please use the exit keywords \"I don't know === <description of problem you are unable to solve>\""),
        "3" => Some("Before providing a solution, ask me to clarify any ambiguous requirements or provide additional context that would help you give a more accurate and useful response."),
        "4" => Some("Break down your response into clear, actionable steps. Number each step and explain the reasoning behind important decisions."),
        "5" => Some("Provide concrete examples in your response. Show don't just tell - include code snippets, sample data, or real-world scenarios that demonstrate your points."),
        "6" => Some("Include validation steps or criteria I can use to verify that your solution works correctly and meets the requirements."),
        "7" => Some("Address potential error cases and edge conditions. Include error handling strategies and common failure points to watch for."),
        "8" => Some("Structure your response as if it were documentation - include clear headings, prerequisites, and any assumptions you're making."),
        "9" => Some("Include suggestions for how to test the solution you're providing. Mention both positive test cases and edge cases to verify."),
        _ => None,
    }
}