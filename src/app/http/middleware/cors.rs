use rustbasic_core::tower_http::cors::{Any, CorsLayer};
use rustbasic_core::axum::http::Method;

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
}
