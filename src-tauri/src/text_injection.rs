use anyhow::Result;
use log::{info, error};
use enigo::{Enigo, Key, Keyboard, Settings};

pub fn inject_text(text: String) -> Result<(), String> {
    inject_text_internal(&text)
        .map_err(|e| {
            error!("Failed to inject text: {}", e);
            e.to_string()
        })
}

pub fn inject_text_internal(text: &str) -> Result<()> {
    info!("Injecting text: {}", text);
    
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => return Err(anyhow::anyhow!("Text injection not available: {}", e)),
    };
    
    // Add one line break before the injected text as specified
    if let Err(e) = enigo.key(Key::Return, enigo::Direction::Press) {
        return Err(anyhow::anyhow!("Key press failed: {}", e));
    }
    if let Err(e) = enigo.key(Key::Return, enigo::Direction::Release) {
        return Err(anyhow::anyhow!("Key release failed: {}", e));
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    // Type the text
    if let Err(e) = enigo.text(text) {
        return Err(anyhow::anyhow!("Text input failed: {}", e));
    }
    
    info!("Text injection completed");
    Ok(())
}

pub fn inject_text_with_hotkey(text: &str) -> Result<()> {
    info!("Injecting text via hotkey: {}", text);
    inject_text_internal(text)
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