/* ---------------------------------------------------------
 * 📑 LABEL: WELCOME CONTROLLER (welcome_controller.rs)
 * Menangani logika tampilan halaman awal.
 * --------------------------------------------------------- */

use axum::response::Response;
use minijinja::context;
use crate::app::render;

pub async fn index() -> Response {
    // Render file "welcome.html" dengan data title
    render("welcome.html", context! {
        title => "Selamat Datang di RustBasic",
    })
}
