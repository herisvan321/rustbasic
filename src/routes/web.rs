use axum::{Router, routing::get};
use crate::app::http::controllers::welcome_controller;
use crate::config::server::AppState;

pub fn router() -> Router<AppState> {
            

            

            

        Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))
        
        
        
        
        
              
        
}
