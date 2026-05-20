use crate::app::inertia;
use rustbasic_core::requests::Request;
use rustbasic_core::responses::ResponseHelper;
use rustbasic_core::server::AppState;
use rustbasic_core::axum::extract::State;
use rustbasic_core::axum::response::IntoResponse;
use rustbasic_core::serde_json::json;

pub async fn index(req: Request) -> impl IntoResponse {
    // Cek apakah fitur Auth sudah terinstal (scaffolded)
    let auth_installed = std::path::Path::new("src/app/http/controllers/auth").exists();

    // Render komponen "Welcome" melalui Inertia dengan data props
    inertia(&req, "Welcome", json!({
        "title": "Selamat Datang di RustBasic",
        "auth_installed": auth_installed,
    }))
}

pub async fn about(req: Request) -> impl IntoResponse {
    // Render komponen "About" melalui Inertia dengan data props
    inertia(&req, "About", json!({
        "title": "Tentang RustBasic SPA",
        "description": "Aplikasi ini telah sepenuhnya bermigrasi dari Multi-Page Application (MPA) tradisional berbasis template Minijinja menjadi Single Page Application (SPA) modern yang ditenagai oleh React.js dan Inertia.js pada backend Axum!",
        "version": "1.0.0",
        "backend": "Rust (Axum + SeaORM)",
        "frontend": "React.js + Vite",
        "bridge": "Inertia.js"
    }))
}

pub async fn dev_info(State(state): State<AppState>, _req: Request) -> impl IntoResponse {
    ResponseHelper::json(rustbasic_core::serde_json::json!({
        "status": "success",
        "app_name": state.config.app_name,
        "environment": if state.config.app_debug { "development" } else { "production" },
        "timezone": state.config.app_timezone,
        "rate_limit": state.config.app_limit_request
    }))
}

