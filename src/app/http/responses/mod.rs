/* ---------------------------------------------------------
 * 📑 LABEL: LARAVEL STYLE RESPONSE (app/http/responses/mod.rs)
 * File ini menyediakan helper untuk mengembalikan response.
 * --------------------------------------------------------- */

use axum::{
    response::{Html, IntoResponse, Json, Redirect},
};
use serde::Serialize;
use serde_json;

pub struct Response;

impl Response {
    /// Mengembalikan tampilan HTML (Minijinja)
    pub fn view(html_content: String) -> impl IntoResponse {
        Html(html_content)
    }

    /// Mengembalikan data JSON
    pub fn json<T: Serialize>(data: T) -> impl IntoResponse {
        Json(data)
    }

    /// Melakukan pengalihan (Redirect)
    pub fn redirect(url: &str) -> impl IntoResponse {
        Redirect::to(url)
    }

    /// Mengembalikan pesan sukses sederhana
    pub fn success(message: &str) -> impl IntoResponse {
        Json(serde_json::json!({
            "status": "success",
            "message": message
        }))
    }
}
