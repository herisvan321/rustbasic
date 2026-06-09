use rustbasic_core::{Router, get, AppState};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index)).name("home")
        .route("/about", get(welcome_controller::about)).name("about")
        .route("/dev", get(welcome_controller::dev_info)).name("dev")
        .route("/test/:id", get(welcome_controller::test_param)).name("test.param")
        .route("/test/multi/:p1/:p2/:p3/:p4", get(welcome_controller::test_multi_param)).name("test.multi")
        .route("/inline", get(|| async { "Ini rute dalam satu baris!" })).name("inline")
}