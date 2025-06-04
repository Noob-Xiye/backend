use crate::core::error::{error_list::CORS_CONFIG_ERROR, AppError, AppResult};
use crate::core::settings::Config;
use crate::settings::AppConfig;
use salvo::extra::cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Cors};
use salvo::http::header::{HeaderValue, CONTENT_TYPE};
use salvo::prelude::*;

pub fn cors_middleware(config: &AppConfig) -> Cors {
    let allowed_origins = if config.security.allowed_origins.contains(&"*".to_string()) {
        AllowedOrigins::any()
    } else {
        AllowedOrigins::some_exact(&config.security.allowed_origins)
    };

    let allowed_methods = if config.security.allowed_methods.contains(&"*".to_string()) {
        AllowedMethods::any()
    } else {
        AllowedMethods::some(
            config
                .security
                .allowed_methods
                .iter()
                .filter_map(|m| m.parse().ok()),
        )
    };

    let allowed_headers = if config.security.allowed_headers.contains(&"*".to_string()) {
        AllowedHeaders::any()
    } else {
        AllowedHeaders::some(&config.security.allowed_headers)
    };

    Cors::new()
        .allow_origins(allowed_origins)
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .allow_credentials(true)
        .expose_headers(config.security.exposed_headers.clone())
        .max_age(config.security.max_age)
}

/// 创建 CORS 中间件
pub fn setup_cors(config: &Config) -> Result<Cors, AppError> {
    let allowed_origins: Vec<HeaderValue> = config
        .security
        .allowed_origins
        .iter()
        .filter_map(|url| HeaderValue::from_str(url).ok())
        .collect();

    let allowed_methods: Vec<Method> = config
        .security
        .allowed_methods
        .iter()
        .filter_map(|method| Method::from_bytes(method.as_bytes()).ok())
        .collect();

    let allowed_headers: Vec<HeaderValue> = config
        .security
        .allowed_headers
        .iter()
        .filter_map(|header| HeaderValue::from_str(header).ok())
        .collect();

    let exposed_headers: Vec<HeaderValue> = config
        .security
        .exposed_headers
        .iter()
        .filter_map(|header| HeaderValue::from_str(header).ok())
        .collect();

    let cors = Cors::new()
        .allow_origins(allowed_origins)
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .expose_headers(exposed_headers)
        .allow_credentials(true) // TODO: 根据配置设置
        .max_age(config.security.max_age as u32); // TODO: 根据配置设置

    Ok(cors)
}

// TODO: 添加更详细的错误处理和配置项映射
