#[allow(dead_code)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
