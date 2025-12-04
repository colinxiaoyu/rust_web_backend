pub fn session_key(jti: &str) -> String {
    format!("session:{}", jti)
}
pub fn refresh_key(jti: &str) -> String {
    format!("refresh:{}", jti)
}
pub fn user_sessions_key(user_id: i64) -> String {
    format!("user:{}:sessions", user_id)
} // set of jti
pub fn blacklist_key(jti: &str) -> String {
    format!("blacklist:{}", jti)
}
pub fn user_permissions_key(user_id: i64) -> String {
    format!("user:{}:perms", user_id)
}
