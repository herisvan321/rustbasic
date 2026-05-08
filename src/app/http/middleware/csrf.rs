/* ---------------------------------------------------------
 * 📑 LABEL: CSRF PROTECTION (app/http/middleware/csrf.rs)
 * Melindungi aplikasi dari Cross-Site Request Forgery.
 * --------------------------------------------------------- */

use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
    middleware::Next,
    response::Response,
};
use axum_session::Session;
use rustbasic_core::session_manager::RustBasicSessionStore;
use rand::distr::SampleString;

pub async fn csrf_middleware(
    session: Session<RustBasicSessionStore>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Pastikan ada token CSRF di session
    let token = match session.get::<String>("_token") {
        Some(t) => t,
        None => {
            let new_token = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 40);
            session.set("_token", new_token.clone());
            new_token
        }
    };

    // 2. Validasi untuk request yang mengubah data (POST, PUT, DELETE, dll)
    let method = req.method();
    if method == Method::POST || method == Method::PUT || method == Method::PATCH || method == Method::DELETE {
        // Ambil token dari header atau form
        let header_token = req.headers()
            .get("X-CSRF-TOKEN")
            .and_then(|h| h.to_str().ok());
        
        // Note: Untuk form body, biasanya kita butuh extractor. 
        // Untuk saat ini kita wajibkan via Header X-CSRF-TOKEN (HTMX style).
        
        if let Some(h_token) = header_token {
            if h_token != token {
                return Err(StatusCode::from_u16(419).unwrap());
            }
        } else {
            return Err(StatusCode::from_u16(419).unwrap());
        }
    }

    Ok(next.run(req).await)
}
