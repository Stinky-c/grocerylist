use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn by_id(user_id: Uuid, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn by_email(email: String, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await
    }
}
