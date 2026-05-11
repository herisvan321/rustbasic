use crate::app::view;
use rustbasic_core::requests::Request;
use rustbasic_core::responses::ResponseHelper;
use rustbasic_core::server::AppState;
use rustbasic_core::axum::extract::State;
use rustbasic_core::axum::response::IntoResponse;
use rustbasic_core::minijinja::context;

pub async fn index(req: Request) -> impl IntoResponse {
    // Cek apakah fitur Auth sudah terinstal (scaffolded)
    let auth_installed = std::path::Path::new("src/app/http/controllers/auth").exists();

    // Render file "welcome.rsx" dengan data title
    view(&req, "welcome.rb.html", context! {
        title => "Selamat Datang di RustBasic",
        auth_installed => auth_installed,
    })
}

pub async fn dev_info(State(state): State<AppState>, _req: Request) -> impl IntoResponse {
    ResponseHelper::json(rustbasic_core::serde_json::json!({
        "status": "success",
        "app_name": state.config.app_name,
        "environment": if state.config.app_debug { "development" } else { "production" },
        "timezone": state.config.app_timezone,
        "rate_limit": state.config.app_limit_request
    }))
}

