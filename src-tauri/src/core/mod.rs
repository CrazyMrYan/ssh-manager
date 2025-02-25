use std::path::PathBuf;
use std::fs;
use std::process::Command;
use openssh_keys::PublicKey;
use chrono::{DateTime, Utc};

pub struct KeyManager {
    key_path: PathBuf,
}

impl KeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let key_path = home.join(".ssh");
        std::fs::create_dir_all(&key_path)?;
        
        Ok(Self {
            key_path,
        })
    }

    pub async fn generate_key(&self, name: &str, key_type: &str, comment: Option<String>, email: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let key_path = self.key_path.join(name);
        
        // 使用 ssh-keygen 生成密钥，对于 RSA 指定 4096 位
        let mut args = vec!["-t", key_type];
        if key_type == "rsa" {
            args.extend(["-b", "4096"]);
        }
        
        // 添加注释（如果提供）
        let comment_value = if let Some(email_val) = &email {
            format!("{}", email_val)
        } else if let Some(comment_val) = &comment {
            comment_val.clone()
        } else {
            "".to_string()
        };
        
        if !comment_value.is_empty() {
            args.extend(["-C", &comment_value]);
        }
        
        args.extend(["-f", key_path.to_str().unwrap(), "-N", ""]);

        let output = Command::new("ssh-keygen")
            .args(&args)
            .output()?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }

        // 设置正确的权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600))?;
            std::fs::set_permissions(&key_path.with_extension("pub"), std::fs::Permissions::from_mode(0o644))?;
        }

        // 更新 SSH 配置文件
        self.update_ssh_config(name)?;

        Ok(())
    }

    fn update_ssh_config(&self, key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.key_path.join("config");
        let mut config_content = String::new();
        
        if config_path.exists() {
            config_content = fs::read_to_string(&config_path)?;
        }

        // 添加新的配置
        let key_path = self.key_path.join(key_name);
        let new_config = format!(
            "\nHost *\n  IdentityFile {}\n",
            key_path.to_str().unwrap()
        );

        if !config_content.contains(&new_config) {
            fs::write(&config_path, config_content + &new_config)?;
            
            // 设置配置文件权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600))?;
            }
        }

        Ok(())
    }

    pub async fn list_keys(&self) -> Result<Vec<SSHKey>, Box<dyn std::error::Error>> {
        let mut keys = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.key_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "pub" {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(pubkey) = PublicKey::parse(&content) {
                                    let name = path.file_stem()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("unknown")
                                        .to_string();

                                    let private_key_path = path.with_extension("");
                                    let metadata = fs::metadata(&private_key_path)?;
                                    let last_modified: DateTime<Utc> = metadata.modified()?.into();

                                    keys.push(SSHKey {
                                        name,
                                        fingerprint: pubkey.fingerprint(),
                                        last_used: last_modified.to_rfc3339(),
                                        key_type: pubkey.keytype().to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(keys)
    }

    pub async fn delete_key(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let private_key = self.key_path.join(name);
        let public_key = private_key.with_extension("pub");

        if private_key.exists() {
            fs::remove_file(&private_key)?;
        }
        if public_key.exists() {
            fs::remove_file(&public_key)?;
        }

        // 从 SSH 配置中移除
        self.remove_from_ssh_config(name)?;

        Ok(())
    }

    fn remove_from_ssh_config(&self, key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.key_path.join("config");
        if !config_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&config_path)?;
        let key_path = self.key_path.join(key_name).to_str().unwrap().to_string();
        
        let new_content: String = content
            .lines()
            .filter(|line| !line.contains(&key_path))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(config_path, new_content)?;
        Ok(())
    }

    pub async fn get_public_key(&self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let public_key_path = self.key_path.join(format!("{}.pub", name));
        if !public_key_path.exists() {
            return Err("Public key not found".into());
        }
        Ok(fs::read_to_string(public_key_path)?)
    }
}

#[derive(serde::Serialize)]
pub struct SSHKey {
    pub name: String,
    pub fingerprint: String,
    pub last_used: String,
    pub key_type: String,
} 