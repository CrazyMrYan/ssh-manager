use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::App;
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub git_config: GitConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxySettings {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        create_default_config(&config_path)?;
    }
    Ok(())
}

fn create_default_config(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        git_config: GitConfig {
            user_name: String::new(),
            user_email: String::new(),
        },
    };
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home.join(".ssh-manager").join("config.json"))
}

#[allow(dead_code)]
fn get_git_global_config(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--global", key])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
#[allow(dead_code)]
pub async fn get_git_config() -> Result<Config, String> {
    let user_name = get_git_global_config("user.name")
        .map_err(|e| e.to_string())?;
    let user_email = get_git_global_config("user.email")
        .map_err(|e| e.to_string())?;

    Ok(Config {
        git_config: GitConfig {
            user_name,
            user_email,
        },
    })
}

#[tauri::command]
#[allow(dead_code)]
pub async fn save_git_config(config: Config) -> Result<(), String> {
    // 设置全局 Git 配置
    Command::new("git")
        .args(["config", "--global", "user.name", &config.git_config.user_name])
        .output()
        .map_err(|e| e.to_string())?;

    Command::new("git")
        .args(["config", "--global", "user.email", &config.git_config.user_email])
        .output()
        .map_err(|e| e.to_string())?;

    Ok(())
} 