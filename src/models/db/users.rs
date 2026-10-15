#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
