use bcrypt::{hash, verify, BcryptError};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, 12) // 12 是成本因子，越高越慢但越安全
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(password, hashed_password)
}
