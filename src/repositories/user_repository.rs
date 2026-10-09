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

pub async fn email_exists(db: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
    return sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
        .bind(email)
        .fetch_one(db)
        .await;
}

pub async fn username_exists(db: &PgPool, username: &str) -> Result<bool, sqlx::Error> {
    return sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)")
        .bind(username)
        .fetch_one(db)
        .await;
}

pub async fn get_user_from_email(db: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    return sqlx::query_as::<_, User>(
        "SELECT id,username,email,password_hash,created_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_one(db)
    .await;
}
