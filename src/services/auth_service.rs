use sqlx::PgPool;

use crate::{
    AppError, models::user::User, repositories::user_repository, utils::password::hash_password,
};

pub async fn register(
    db: &PgPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<User, AppError> {
    let password_hash = hash_password(password)?;
    let user = user_repository::create(db, email, username, &password_hash).await?;
    Ok(user)
}
