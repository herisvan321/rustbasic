/* ---------------------------------------------------------
 * 📑 LABEL: PENGATURAN RUTE (web.rs)
 * File ini menghubungkan URL dengan fungsi di Controller.
 * --------------------------------------------------------- */

use axum::{Router, routing::get};
use crate::app::http::controllers::{welcome_controller, test_controller};

pub fn router() -> Router {
    Router::new()
        // URL "/" akan ditangani oleh welcome_controller::index
        .route("/", get(welcome_controller::index))
        .route("/test", get(welcome_controller::test_request))
        
        // CSRF Demo
        .route("/test-csrf", get(test_controller::csrf_form))
        .route("/test-csrf", axum::routing::post(test_controller::csrf_submit))
}
