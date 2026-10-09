use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserRefreshToken {
    pub id: Uuid,
    pub token_hash: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub replace_by_token_id: Option<Uuid>,
}
