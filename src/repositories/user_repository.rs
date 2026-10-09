use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub async fn create(
    db: &PgPool,
    email: &str,
    username: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    return sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id,username,email,password_hash)
        VALUES($1,$2,$3,$4)
        RETURNING id,username,password_hash,email,created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(db)
    .await;
}
