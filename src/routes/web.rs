use rustbasic_core::{Router, get, AppState};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/about", get(welcome_controller::about))
        .route("/dev", get(welcome_controller::dev_info))
        
        
        .route("/inline", get(|| async { "Ini rute dalam satu baris!" }))
}