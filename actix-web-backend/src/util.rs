use actix_web::rt::task;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    task::spawn_blocking(move || {
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|v| v.to_string())
            .map_err(|e| anyhow::anyhow!(e))
    })
    .await?
}

pub async fn verify_password(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    })
    .await?
}
