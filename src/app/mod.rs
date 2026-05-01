/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Mengatur template engine (Minijinja) dan fungsi render.
 * --------------------------------------------------------- */

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use minijinja::{Environment, path_loader};
use std::sync::LazyLock;

pub mod http;

// 1. Setup Engine Template (Minijinja)
pub static JINJA: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("resources/views"));
    env
});

// 2. Fungsi Helper untuk Render HTML
pub fn render(template: &str, context: minijinja::Value) -> Response {
    match JINJA.get_template(template) {
        Ok(tmpl) => match tmpl.render(context) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(err) => {
                tracing::error!("Gagal render template: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
        Err(err) => {
            tracing::error!("Template tidak ditemukan: {}", err);
            (StatusCode::NOT_FOUND, "Template Not Found").into_response()
        }
    }
}
