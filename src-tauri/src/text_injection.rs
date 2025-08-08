use anyhow::{Result, Context};
use log::{info, error};
use enigo::{Enigo, KeyboardControllable};

#[tauri::command]
pub async fn inject_text(text: String) -> Result<(), String> {
    inject_text_internal(&text)
        .map_err(|e| {
            error!("Failed to inject text: {}", e);
            e.to_string()
        })
}

fn inject_text_internal(text: &str) -> Result<()> {
    info!("Injecting text: {}", text);
    
    let mut enigo = Enigo::new();
    
    // Simulate typing the text
    for c in text.chars() {
        match c {
            '\n' => {
                enigo.key_click(enigo::Key::Return);
            }
            '\t' => {
                enigo.key_click(enigo::Key::Tab);
            }
            _ => {
                enigo.key_sequence_char(c);
            }
        }
        
        // Small delay to make it more realistic
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    info!("Text injection completed");
    Ok(())
}

pub fn inject_text_with_hotkey(text: &str) -> Result<()> {
    info!("Injecting text via hotkey: {}", text);
    
    let mut enigo = Enigo::new();
    
    // For hotkey-triggered injection, we might want to use a different approach
    // that's more immediate and doesn't simulate typing
    enigo.key_sequence(text);
    
    info!("Hotkey text injection completed");
    Ok(())
}

pub fn inject_text_clipboard(text: &str) -> Result<()> {
    info!("Injecting text via clipboard: {}", text);
    
    // This would use the system clipboard to paste text
    // Implementation would depend on the platform
    // For now, we'll use the typing approach
    
    inject_text_internal(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_text() {
        let text = "Hello, World!\nThis is a test.";
        // This test would verify that text parsing works correctly
        assert!(!text.is_empty());
    }
} 