use rustbasic_core::{Router, get, AppState};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index)).name("home")
        .route("/about", get(welcome_controller::about)).name("about")
}