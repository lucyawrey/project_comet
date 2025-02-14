use crate::model::fields::AccessLevel;

pub fn hash_password() {}

/// Used to has `session_token`, `access_token`, and `recovery_code`.
pub fn hash_token() {}

pub fn generate_password() {}

pub fn generate_session_token() {}

/// Returned tokens have the format `default|server:gameserverid|admin_IdBase64Representation_secret`.
pub fn generate_access_token(
    access_token_id: i64,
    access_level: AccessLevel,
    game_server_id: Option<String>,
) {
}

pub fn generate_recovery_code() {}
