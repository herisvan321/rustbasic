/* ---------------------------------------------------------
 * 📑 LABEL: AUTH MIDDLEWARE (middleware/auth.rs)
 * Melindungi rute yang membutuhkan login.
 * --------------------------------------------------------- */

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_session::Session;
use rustbasic_core::session_manager::RustBasicSessionStore;

pub async fn auth_middleware(
    session: Session<RustBasicSessionStore>,
    req: Request,
    next: Next,
) -> Response {
    // Cek apakah ada user_id di session
    let user_id = session.get::<i64>("user_id");

    if user_id.is_none() {
        // Simpan pesan flash dan arahkan ke login
        session.set("flash_error", "Silakan login terlebih dahulu.");
        return Redirect::to("/login").into_response();
    }

    next.run(req).await
}

pub async fn guest_middleware(
    session: Session<RustBasicSessionStore>,
    req: Request,
    next: Next,
) -> Response {
    // Kebalikan dari auth: jangan izinkan user yang sudah login akses halaman ini (misal login/register)
    let user_id = session.get::<i64>("user_id");

    if user_id.is_some() {
        return Redirect::to("/").into_response();
    }

    next.run(req).await
}
