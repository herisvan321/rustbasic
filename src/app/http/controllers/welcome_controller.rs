use crate::app::inertia;
use rustbasic_core::requests::Request;
use rustbasic_core::{IntoResponse};
use rustbasic_core::serde_json::json;

pub async fn index(req: Request) -> impl IntoResponse {
    // Cek apakah fitur Auth sudah terinstal (scaffolded) menggunakan cfg flag compile-time
    let auth_installed = cfg!(breeze);
    let user_id = req.session.get::<i32>("user_id").unwrap_or(0);
    let is_logged_in = user_id > 0;

    // Render komponen "Welcome" melalui Inertia dengan data props
    inertia(&req, "Welcome", json!({
        "title": "Selamat Datang di RustBasic",
        "auth_installed": auth_installed,
        "is_logged_in": is_logged_in,
    }))
}

pub async fn about(req: Request) -> impl IntoResponse {
    // Render komponen "About" melalui Inertia dengan data props
    inertia(&req, "About", json!({
        "title": "Tentang RustBasic SPA",
        "description": "Framework Full-stack modern berbasis Rust dengan React.js dan Inertia.js.",
        "version": env!("CARGO_PKG_VERSION"),
        "backend": "Rust (Custom HTTP Engine)",
        "frontend": "React.js + Vite",
        "bridge": "Inertia.js"
    }))
}
