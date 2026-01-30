use sha2::{Digest, Sha256};

/// Generate a unique device hash based on machine ID
pub fn get_device_hash() -> String {
    match machine_uid::get() {
        Ok(uid) => {
            let mut hasher = Sha256::new();
            hasher.update(b"bar-tomato-");
            hasher.update(uid.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
        Err(_) => {
            // Fallback: use hostname + username
            let hostname = hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            let username = whoami::username_os()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            let mut hasher = Sha256::new();
            hasher.update(b"bar-tomato-fallback-");
            hasher.update(hostname.as_bytes());
            hasher.update(username.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
    }
}

// Helper for hex encoding
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
