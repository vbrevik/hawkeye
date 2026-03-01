use rand::RngExt;
use sha2::{Digest, Sha256};

const KEY_RANDOM_LEN: usize = 32;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_api_key() -> String {
    let mut rng = rand::rng();
    let random: String = (0..KEY_RANDOM_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("hke_{}", random)
}

pub fn hash_api_key(key: &str) -> String {
    let result = Sha256::digest(key.as_bytes());
    format!("sha256:{:x}", result)
}

pub fn key_prefix(key: &str) -> String {
    key.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_key_format() {
        let key = generate_api_key();
        assert!(key.starts_with("hke_"));
        assert_eq!(key.len(), 36); // hke_ (4) + 32 random chars
    }

    #[test]
    fn test_generate_api_key_uniqueness() {
        let key1 = generate_api_key();
        let key2 = generate_api_key();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_hash_api_key_deterministic() {
        let hash1 = hash_api_key("hke_testkey123");
        let hash2 = hash_api_key("hke_testkey123");
        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_hash_api_key_different_keys() {
        let hash1 = hash_api_key("hke_key1");
        let hash2 = hash_api_key("hke_key2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_key_prefix() {
        let prefix = key_prefix("hke_abcd1234rest");
        assert_eq!(prefix, "hke_abcd");
    }
}
