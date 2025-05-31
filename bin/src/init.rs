use salvo::catcher::Catcher;
use salvo::compression::{Compression, CompressionLevel};
use salvo::Router;
use salvo::{cors::Cors, http::Method, prelude::*};
use thinkrs_ai::get_ai_router;
use thinkrs_core::middlewares::*;
use thinkrs_system::get_system_router;

pub async fn create_service() -> salvo::Service {
    let router = get_app_router();
    let doc = OpenApi::new("thinkrs api", "0.0.1").merge_router(&router);
    let router = Router::new()
        .append(&mut vec![router])
        .append(&mut vec![Router::new()
            .push(doc.into_router("/api-doc/openapi.json"))
            .push(
                SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"),
            )]);
    let cors = Cors::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:3000")
        .allow_headers(vec!["Authorization", "content-type"])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .into_handler();
    Service::new(router)
        .hoop(cors)
        .hoop(Compression::new().enable_gzip(CompressionLevel::Default))
        .catcher(Catcher::default())
}

fn get_app_router() -> Router {
    let mut routers = get_system_router();
    let mut ai_routers = get_ai_router();
    let router = Router::new()
        .append(&mut routers)
        .append(&mut ai_routers)
        .hoop_when(jwt_middleware, |req: &Request, depot: &Depot| -> bool {
            need_auth(req, depot)
        })
        .hoop_when(
            normalized_response_middleware,
            |req: &Request, _depot: &Depot| -> bool { req.uri().path().contains("/api/") },
        )
        .hoop(request_id_middleware)
        .hoop(logger_middleware);
    router
}

const NO_AUTH_PATHS: &[&str] = &["/api/login", "/api/refresh_token","/api/img/","/api/patients/mobile/","/api/patient/"];

fn need_auth(req: &Request, _depot: &Depot) -> bool {
    let uri = req.uri().path();
    let needs_auth = uri.contains("/api/") && !NO_AUTH_PATHS.iter().any(
        |path| uri.starts_with(path)
    );
    dbg!(needs_auth);
    needs_auth
}