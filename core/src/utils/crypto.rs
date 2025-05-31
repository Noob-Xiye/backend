use bcrypt::{hash, verify, BcryptError};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, 12) // 12 is the cost factor, higher is slower but more secure
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(password, hashed_password)
}
