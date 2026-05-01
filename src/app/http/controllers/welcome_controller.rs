use axum::response::Response;
use minijinja::context;
use crate::app::render;

pub async fn index() -> Response {
    render("welcome.html", context! {
        title => "Welcome to Rust Axum",
    })
}
