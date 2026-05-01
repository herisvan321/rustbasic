use axum::{Router, routing::get};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router {
    Router::new()
        .route("/", get(welcome_controller::index))
}
