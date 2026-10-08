use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString,
    },
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}
