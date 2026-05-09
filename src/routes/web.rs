use rustbasic_core::axum::{Router, routing::get};
use rustbasic_core::server::AppState;
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))
        
        
        
        
        
        
        
        
        
             
}
