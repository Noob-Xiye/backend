use crate::settings::AppConfig;
use salvo::extra::cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Cors};
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
