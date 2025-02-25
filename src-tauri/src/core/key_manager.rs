use std::path::PathBuf;
use openssh_keys::{KeyPair, KeyType, PublicKey};
use crate::security::SecurityManager;

pub struct KeyManager {
    security: SecurityManager,
    key_path: PathBuf,
}

impl KeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let key_path = home.join(".ssh");
        std::fs::create_dir_all(&key_path)?;
        
        Ok(Self {
            security: SecurityManager::new(),
            key_path,
        })
    }

    pub async fn generate_key(&self, name: &str, key_type: KeyType) -> Result<KeyPair, Box<dyn std::error::Error>> {
        let key_pair = KeyPair::generate(key_type)?;
        let path = self.key_path.join(name);
        
        // 保存私钥
        std::fs::write(&path, key_pair.serialize_private()?)?;
        std::fs::write(path.with_extension("pub"), key_pair.serialize_public()?)?;
        
        // 设置正确的权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }
        
        Ok(key_pair)
    }
} 