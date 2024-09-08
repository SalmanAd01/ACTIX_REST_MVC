use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}