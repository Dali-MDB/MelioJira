use argon2::{
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,rand_core::OsRng,},
    Argon2,
};

pub fn hash_password(password: &str)->Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}

pub fn verify_password( password: &str, stored_hash: &str,)->Result<bool, Error> {
    let parsed_hash = PasswordHash::new(stored_hash)?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash,)?;
    Ok(true)
}