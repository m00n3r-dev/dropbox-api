use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user_refresh_token::UserRefreshToken;

pub async fn create(
    db: &PgPool,
    token_hash: &str,
    user_id: &Uuid,
    expires_at: &i64,
    replace_by_token_id: Option<&Uuid>,
) -> Result<UserRefreshToken, sqlx::Error> {
    return sqlx::query_as::<_, UserRefreshToken>(
        r#"
        INSERT INTO user_refresh_tokens (token_hash,user_id,expires_at,replace_by_token_id)
        VALUES ($1,$2,$3,$4)
        RETURNING id,user_id,expires_at
        "#,
    )
    .bind(token_hash)
    .bind(user_id)
    .bind(expires_at)
    .bind(replace_by_token_id)
    .fetch_one(db)
    .await;
}
