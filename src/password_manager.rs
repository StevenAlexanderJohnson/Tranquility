use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Error hashing password: {:?}", e))?
        .to_string())
}

pub fn validate_password(password: &str, hashed: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let hashed = PasswordHash::new(hashed)
        .map_err(|e| format!("Error processing hashed password: {:?}", e))?;
    match Argon2::default().verify_password(password.as_bytes(), &hashed) {
        Ok(()) => Ok(true),
        Err(e) => Err(format!("Error validating password: {:?}", e).into()),
    }
}
