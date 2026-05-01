/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Mengatur template engine (Minijinja) dan fungsi render.
 * --------------------------------------------------------- */

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use minijinja::{Environment, path_loader, context};
use std::sync::LazyLock;
use crate::app::http::requests::Request as AppRequest;
use serde_json::{json, Value};

pub mod http;

// 1. Setup Engine Template (Minijinja)
pub static JINJA: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("resources/views"));
    env
});

// 2. Fungsi Helper untuk Render HTML Statis
pub fn render(template: &str, context: minijinja::Value) -> Response {
    render_internal(template, context)
}

// 3. Fungsi Helper untuk Render dengan Session (Laravel Style)
pub fn view(req: &AppRequest, template: &str, ctx: minijinja::Value) -> Response {
    // Kita konversi minijinja::Value ke serde_json::Value untuk manipulasi mudah
    let mut ctx_value = serde_json::to_value(&ctx).unwrap_or_else(|_| json!({}));
    
    if !ctx_value.is_object() {
        ctx_value = json!({});
    }
    
    let obj = ctx_value.as_object_mut().unwrap();

    // Inisialisasi default agar tidak error "undefined value" di template
    if !obj.contains_key("errors") {
        obj.insert("errors".to_string(), json!({}));
    }
    if !obj.contains_key("old") {
        obj.insert("old".to_string(), json!({}));
    }
    if !obj.contains_key("flash_success") {
        obj.insert("flash_success".to_string(), json!(""));
    }
    if !obj.contains_key("flash_error") {
        obj.insert("flash_error".to_string(), json!(""));
    }

    // Ambil Data dari Session (Flash Message, Errors, Old Input)
    if let Some(success) = req.session.get::<String>("flash_success") {
        obj.insert("flash_success".to_string(), json!(success));
        req.session.remove("flash_success");
    }
    if let Some(error) = req.session.get::<String>("flash_error") {
        obj.insert("flash_error".to_string(), json!(error));
        req.session.remove("flash_error");
    }
    if let Some(errors) = req.session.get::<Value>("errors") {
        obj.insert("errors".to_string(), errors);
        req.session.remove("errors");
    }
    if let Some(old) = req.session.get::<Value>("old_input") {
        obj.insert("old".to_string(), old);
    }

    // Tambahkan CSRF Token
    if let Some(token) = req.session.get::<String>("_token") {
        obj.insert("csrf_token".to_string(), json!(token));
    }

    // Tambahkan status login
    let is_logged_in = req.session.get::<i64>("user_id").is_some();
    obj.insert("auth".to_string(), json!(is_logged_in));

    render_internal(template, minijinja::Value::from_serialize(obj))
}

fn render_internal(template: &str, context: minijinja::Value) -> Response {
    match JINJA.get_template(template) {
        Ok(tmpl) => match tmpl.render(context) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(err) => {
                tracing::error!("Gagal render template: {}", err);
                match JINJA.get_template("errors/minimal.html") {
                    Ok(tmpl) => match tmpl.render(context! { code => 500, title => "Server Error", message => "Terjadi kesalahan saat memproses tampilan." }) {
                        Ok(rendered) => Html(rendered).into_response(),
                        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Critical Render Error").into_response(),
                    },
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
                }
            }
        },
        Err(err) => {
            tracing::error!("Template tidak ditemukan: {}", err);
            match JINJA.get_template("errors/minimal.html") {
                Ok(tmpl) => match tmpl.render(context! { code => 404, title => "Page Not Found", message => "Halaman atau template tidak ditemukan." }) {
                    Ok(rendered) => Html(rendered).into_response(),
                    Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
                },
                Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            }
        }
    }
}
