use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

/// 对密码进行 bcrypt 加密
pub fn hash_password(password: &str) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

/// 验证密码是否与哈希匹配
pub fn verify_password(password: &str, hashed_password: &str) -> BcryptResult<bool> {
    verify(password, hashed_password)
}

// TODO: 考虑使用更安全的密码哈希算法或加盐策略
