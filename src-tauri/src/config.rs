use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub title: String,
    pub text: String,
    pub description: String,
    pub hotkey: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiSettings {
    pub always_on_top: bool,
    pub transparency: f64,
    pub position_x: String,
    pub auto_start: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub global_hotkeys: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub snippets: Vec<Snippet>,
    pub settings: Settings,
    pub ui_settings: UiSettings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            snippets: vec![],
            settings: Settings {
                auto_start: false,
                minimize_to_tray: true,
                global_hotkeys: true,
            },
            ui_settings: UiSettings {
                always_on_top: true,
                transparency: 0.9,
                position_x: "top-right".to_string(),
                auto_start: false,
            },
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let app_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("enhancekeys");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("config.json"))
}

pub fn init_config() -> Result<()> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        info!("Creating default configuration");
        let mut default_config = Config::default();
        
        // Load default snippets from TOML file
        match load_default_snippets() {
            Ok(snippets) => {
                default_config.snippets = snippets;
                info!("Loaded {} default snippets", default_config.snippets.len());
            },
            Err(e) => {
                error!("Failed to load default snippets: {}", e);
            }
        }
        
        save_config_internal(&default_config)?;
    }
    
    Ok(())
}

fn load_config_internal() -> Result<Config> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(Config::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .context("Failed to read config file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

fn save_config_internal(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    fs::write(&config_path, content)
        .context("Failed to write config file")?;
    
    Ok(())
}

pub fn load_config() -> Result<Config, String> {
    load_config_internal()
        .map_err(|e| {
            error!("Failed to load config: {}", e);
            e.to_string()
        })
}

pub fn save_config(config: Config) -> Result<(), String> {
    save_config_internal(&config)
        .map_err(|e| {
            error!("Failed to save config: {}", e);
            e.to_string()
        })
}

pub fn load_default_snippets() -> Result<Vec<Snippet>> {
    let default_snippets_path = PathBuf::from("config/default_snippets.toml");
    
    if !default_snippets_path.exists() {
        return Ok(vec![]);
    }
    
    let content = fs::read_to_string(&default_snippets_path)
        .context("Failed to read default snippets file")?;
    
    let parsed: toml::Value = toml::from_str(&content)
        .context("Failed to parse default snippets file")?;
    
    let mut snippets = Vec::new();
    
    // Load snippets based on button_1 through button_9 format
    for i in 1..=9 {
        let button_key = format!("button_{}", i);
        
        if let (Some(snippets_table), Some(hotkeys_table)) = (
            parsed.get("snippets").and_then(|v| v.as_table()),
            parsed.get("hotkeys").and_then(|v| v.as_table())
        ) {
            let title_key = format!("{}_title", button_key);
            let text_key = format!("{}_text", button_key);
            let desc_key = format!("{}_description", button_key);
            
            if let (Some(title), Some(text), Some(description), Some(hotkey)) = (
                snippets_table.get(&title_key).and_then(|v| v.as_str()),
                snippets_table.get(&text_key).and_then(|v| v.as_str()),
                snippets_table.get(&desc_key).and_then(|v| v.as_str()),
                hotkeys_table.get(&button_key).and_then(|v| v.as_str()),
            ) {
                snippets.push(Snippet {
                    id: button_key,
                    title: title.to_string(),
                    text: text.to_string(),
                    description: description.to_string(),
                    hotkey: hotkey.to_string(),
                });
            }
        }
    }
    
    Ok(snippets)
} 