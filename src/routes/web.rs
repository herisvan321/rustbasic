/* ---------------------------------------------------------
 * 📑 LABEL: PENGATURAN RUTE (web.rs)
 * File ini menghubungkan URL dengan fungsi di Controller.
 * --------------------------------------------------------- */

use axum::{Router, routing::{get, post}, middleware::from_fn};
use crate::app::http::controllers::{welcome_controller, test_controller, auth, dashboard_controller};
use crate::app::http::middleware::auth::{auth_middleware, guest_middleware};

pub fn router() -> Router {
    let guest_routes = Router::new()
        .route("/login", get(auth::auth_controller::AuthController::login_page))
        .route("/login", post(auth::auth_controller::AuthController::login))
        .route("/register", get(auth::auth_controller::AuthController::register_page))
        .route("/register", post(auth::auth_controller::AuthController::register))
        .layer(from_fn(guest_middleware));

    let auth_routes = Router::new()
        .route("/dashboard", get(dashboard_controller::DashboardController::index))
        .route("/logout", post(auth::auth_controller::AuthController::logout))
        .route("/test", get(welcome_controller::test_request))
        .route("/test-csrf", get(test_controller::csrf_form))
        .route("/test-csrf", post(test_controller::csrf_submit))
        .layer(from_fn(auth_middleware));

    Router::new()
        .route("/", get(welcome_controller::index))
        .merge(guest_routes)
        .merge(auth_routes)
}
