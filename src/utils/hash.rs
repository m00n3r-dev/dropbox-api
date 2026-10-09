use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher};

pub fn hash_string(password: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string())
}

pub fn verify_hash(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
