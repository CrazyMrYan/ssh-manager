#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::core::{KeyManager, SSHKey};
use crate::config::{Config, get_git_config, save_git_config};
use tauri::Manager;
use std::fs;
use std::collections::HashMap;

mod core;
mod security;
mod config;

#[tauri::command]
async fn get_ssh_keys() -> Result<Vec<SSHKey>, String> {
    let key_manager = KeyManager::new().map_err(|e| e.to_string())?;
    key_manager.list_keys().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn generate_ssh_key(name: String, key_type: String, comment: Option<String>, email: Option<String>) -> Result<(), String> {
    let key_manager = KeyManager::new().map_err(|e| e.to_string())?;
    key_manager.generate_key(&name, &key_type, comment, email).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_ssh_key(name: String) -> Result<(), String> {
    let key_manager = KeyManager::new().map_err(|e| e.to_string())?;
    key_manager.delete_key(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn copy_public_key(name: String) -> Result<(), String> {
    let key_manager = KeyManager::new().map_err(|e| e.to_string())?;
    let public_key = key_manager.get_public_key(&name).await.map_err(|e| e.to_string())?;
    
    // 使用 clipboard-manager 复制到剪贴板
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(public_key).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_git_configs() -> Result<HashMap<String, String>, String> {
    let output = std::process::Command::new("git")
        .args(["config", "--global", "--list"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    let config_str = String::from_utf8_lossy(&output.stdout);
    let mut configs = HashMap::new();
    
    for line in config_str.lines() {
        if let Some((key, value)) = line.split_once('=') {
            configs.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    Ok(configs)
}

#[tauri::command]
async fn set_git_config(key: String, value: String) -> Result<(), String> {
    let output = std::process::Command::new("git")
        .args(["config", "--global", &key, &value])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}

#[tauri::command]
async fn delete_git_config(key: String) -> Result<(), String> {
    let output = std::process::Command::new("git")
        .args(["config", "--global", "--unset", &key])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}

#[tauri::command]
async fn get_public_key(name: String) -> Result<String, String> {
    let key_manager = KeyManager::new().map_err(|e| e.to_string())?;
    key_manager.get_public_key(&name).await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_ssh_keys,
            generate_ssh_key,
            delete_ssh_key,
            copy_public_key,
            get_all_git_configs,
            set_git_config,
            delete_git_config,
            get_public_key,
            get_git_config,
            save_git_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 