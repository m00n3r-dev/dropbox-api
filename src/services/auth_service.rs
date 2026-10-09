use sqlx::PgPool;

use crate::{
    AppError,
    models::user::User,
    repositories::user_repository,
    utils::password::{hash_password, verify_password},
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

    let password_hash = hash_password(password)?;
    let user = user_repository::create(db, email, username, &password_hash).await?;
    Ok(user)
}

pub async fn login(db: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let email_exists = user_repository::email_exists(db, email).await?;
    if !email_exists {
        return Err(AppError::BadRequest("Email or password wrong".into()));
    }

    let user = user_repository::get_user_from_email(db, email).await?;

    verify_password(password, &user.password_hash)
        .map_err(|_| AppError::BadRequest("Email or password wrong".into()))?;

    Ok(user)
}
