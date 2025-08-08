// EnhanceKeys - Global Hotkey Application
use std::thread;
use std::time::Duration;

mod config;
mod text_injection;
mod system_integration;
mod hotkeys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🚀 EnhanceKeys - Prompt Enhancement Tool");
    println!("═══════════════════════════════════════");
    println!("✅ Starting with global hotkey support...");
    
    // Initialize configuration
    if let Err(e) = config::init_config() {
        println!("⚠️  Configuration warning: {}", e);
    } else {
        println!("✅ Configuration system initialized");
    }

    // Initialize system integration
    if let Err(e) = system_integration::init() {
        println!("⚠️  System integration warning: {}", e);
    } else {
        println!("✅ System integration initialized");
    }

    // Initialize hotkeys
    match hotkeys::init_hotkeys() {
        Ok(_) => {
            println!("✅ Global hotkeys registered successfully!");
            println!("🔥 Alt+Shift+1-9 hotkeys are now ACTIVE");
        },
        Err(e) => {
            println!("⚠️  Global hotkeys failed: {}", e);
            println!("💡 This is expected in WSL - works on native Windows/Linux");
        }
    }

    println!("═══════════════════════════════════════");
    println!("📋 Available hotkeys (Alt+Shift+Number):");
    println!("  1️⃣  Alt+Shift+1 - Ask Questions");
    println!("  2️⃣  Alt+Shift+2 - Need Review");  
    println!("  3️⃣  Alt+Shift+3 - More Context");
    println!("  4️⃣  Alt+Shift+4 - Step by Step");
    println!("  5️⃣  Alt+Shift+5 - Examples");
    println!("  6️⃣  Alt+Shift+6 - Validation");
    println!("  7️⃣  Alt+Shift+7 - Error Handling");
    println!("  8️⃣  Alt+Shift+8 - Documentation");
    println!("  9️⃣  Alt+Shift+9 - Testing");
    println!("═══════════════════════════════════════");
    println!("💡 USAGE:");
    println!("1. Open any text editor (notepad, browser, etc.)");
    println!("2. Click in a text field");
    println!("3. Press Alt+Shift+1 (or any number 1-9)");
    println!("4. The prompt enhancement text will be injected!");
    println!("═══════════════════════════════════════");
    println!("🎯 Application Status: RUNNING");
    println!("⏰ Hotkeys active - try Alt+Shift+1 now!");
    println!("📝 Press Ctrl+C to exit");
    
    // Keep the application running to handle hotkeys
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}