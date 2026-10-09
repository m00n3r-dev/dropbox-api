use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    AppError,
    models::user::User,
    repositories::{user_refresh_token_repository, user_repository},
    utils::{
        hash::{hash_string, verify_hash},
        jwt,
    },
};

pub async fn register(
    db: &PgPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<User, AppError> {
    let email_exists = user_repository::email_exists(db, email).await?;
    if email_exists {
        return Err(AppError::Conflict("Email already exists".into()));
    }

    let username_exists = user_repository::username_exists(db, username).await?;
    if username_exists {
        return Err(AppError::Conflict("Username already exists".into()));
    }

    let password_hash = hash_string(password)?;
    let user = user_repository::create(db, email, username, &password_hash).await?;
    Ok(user)
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(db: &PgPool, email: &str, password: &str) -> Result<LoginResponse, AppError> {
    let email_exists = user_repository::email_exists(db, email).await?;
    if !email_exists {
        return Err(AppError::BadRequest("Email or password wrong".into()));
    }

    let user = user_repository::get_user_from_email(db, email).await?;

    verify_hash(password, &user.password_hash)
        .map_err(|_| AppError::BadRequest("Email or password wrong".into()))?;

    let access_token = jwt::create_token(user.id, user.email, user.username)
        .map_err(|_| AppError::Internal("something went wrong!".into()))?;

    let refresh_token = Uuid::new_v4();
    let refresh_token_hash = hash_string(&refresh_token.to_string())?;

    let refresh_token_exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 30;

    user_refresh_token_repository::create(
        db,
        &refresh_token_hash,
        &user.id,
        &refresh_token_exp,
        None,
    )
    .await?;

    Ok(LoginResponse {
        access_token,
        refresh_token: refresh_token.to_string(),
    })
}
