use crate::model::fields::AccessLevel;
use base32ct::Encoding;
use rand::{rngs::OsRng, TryRngCore};
use sha2::{Digest, Sha256};

pub fn hash_password() {}

/// Used to has `session_token`, `access_token`, and `recovery_code`.
pub fn hash_token(token: &str) -> Option<String> {
    let hash = Sha256::digest(token);
    let mut buf = [0u8; 32];
    let hex_hash = base16ct::lower::encode_str(&hash, &mut buf).ok()?;
    Some(hex_hash.to_string())
}

pub fn get_random_bytes<const COUNT: usize>() -> Option<[u8; COUNT]> {
    let mut bytes = [0u8; COUNT];
    OsRng.try_fill_bytes(&mut bytes).ok()?;
    Some(bytes)
}

pub fn generate_password() {}

pub fn generate_session_token() -> Option<(String, String)> {
    let bytes: [u8; 20] = get_random_bytes()?;
    let mut buf = [0u8; 20];
    let token_base32 = base32ct::Base32::encode(&bytes, &mut buf).ok()?;
    let token_hash = hash_token(token_base32)?;
    Some((token_base32.to_string(), token_hash))
}

/// Returned tokens have the format `default|server:gameserverid|admin_IdBase32Representation_secret`.
pub fn generate_access_token(
    access_token_id: i64,
    access_level: &AccessLevel,
    game_server_id: Option<&str>,
) -> Option<(String, String)> {
    let bytes: [u8; 20] = get_random_bytes()?;
    let mut buf = [0u8; 20];
    let secret_base32 = base32ct::Base32::encode(&bytes, &mut buf).ok()?;
    let mut buf2 = [0u8; 8];
    let id_base32 = base32ct::Base32::encode(&access_token_id.to_be_bytes(), &mut buf2).ok()?;
    let access_level_str = match access_level {
        AccessLevel::Default => "default",
        AccessLevel::GameServer => &format!("server:{}", game_server_id?),
        AccessLevel::Administrator => "admin",
    };
    let token = format!("{}_{}_{}", access_level_str, id_base32, secret_base32);
    let token_hash = hash_token(&token)?;
    Some((token, token_hash))
}

pub fn generate_recovery_code() -> Option<(String, String)> {
    let bytes: [u8; 32] = get_random_bytes()?;
    let mut buf = [0u8; 32];
    let code_base32 = base32ct::Base32::encode(&bytes, &mut buf).ok()?;
    let code_hash = hash_token(code_base32)?;
    Some((code_base32.to_string(), code_hash))
}
