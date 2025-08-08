use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub name: String,
    pub text: String,
    pub hotkey: Option<String>,
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
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let app_dir = tauri::api::path::app_dir(&tauri::Config::default())
        .context("Failed to get app directory")?;
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("config.json"))
}

pub fn init_config() -> Result<()> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        info!("Creating default configuration");
        let default_config = Config::default();
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

#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    load_config_internal()
        .map_err(|e| {
            error!("Failed to load config: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
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
    
    if let Some(snippets_table) = parsed.get("snippets").and_then(|v| v.as_table()) {
        for (id, text) in snippets_table {
            let hotkey = parsed
                .get("hotkeys")
                .and_then(|h| h.get(id))
                .and_then(|h| h.as_str())
                .map(|s| s.to_string());
            
            snippets.push(Snippet {
                id: id.clone(),
                name: id.clone(),
                text: text.as_str().unwrap_or("").to_string(),
                hotkey,
            });
        }
    }
    
    Ok(snippets)
} 