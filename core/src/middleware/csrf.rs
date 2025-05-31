use crate::settings::AppConfig;
use salvo::extra::csrf::CsrfGuard;
use salvo::prelude::*;

pub fn csrf_middleware(config: &AppConfig) -> CsrfGuard {
    // 在实际应用中，密钥应该安全生成和存储。
    // 为了简单起见，这里使用硬编码值。
    let secret = "this_is_a_very_secret_key_change_it_in_production"; // 在生产环境中替换为真实的密钥

    CsrfGuard::new()
        .with_secret(secret.as_bytes())
        .with_httponly(config.session.http_only)
        .with_cookie_name("csrf_token")
        .with_header_name("X-CSRF-Token")
}
