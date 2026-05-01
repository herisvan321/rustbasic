/* ---------------------------------------------------------
 * 📑 LABEL: TEST CONTROLLER (test_controller.rs)
 * Menangani percobaan fitur CSRF dan Input.
 * --------------------------------------------------------- */

use crate::app::render;
use crate::app::http::requests::Request;
use crate::app::http::responses::Response;
use axum::response::IntoResponse;
use axum_session::Session;
use crate::database::session_manager::LaravelSessionStore;
use minijinja::context;

pub async fn csrf_form(session: Session<LaravelSessionStore>) -> impl IntoResponse {
    // Ambil token dari session (dibuat oleh middleware)
    let token = session.get::<String>("_token").unwrap_or_default();
    
    render("test_csrf.html", context! {
        title => "Percobaan CSRF",
        csrf_token => token
    })
}

pub async fn csrf_submit(req: Request) -> impl IntoResponse {
    let name = req.input("name").unwrap_or_else(|| "Kosong".to_string());
    let email = req.input("email").unwrap_or_else(|| "Kosong".to_string());
    
    render("test_csrf_result.html", context! {
        title => "Hasil Pengiriman",
        name => name,
        email => email
    })
}
