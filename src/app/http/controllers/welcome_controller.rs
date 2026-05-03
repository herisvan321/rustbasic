use crate::app::view;
use crate::config::requests::Request;
use crate::config::responses::ResponseHelper;
use crate::config::server::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn index(req: Request) -> impl IntoResponse {
    // Cek apakah fitur Auth sudah terinstal (scaffolded)
    let auth_installed = std::path::Path::new("src/app/http/controllers/auth").exists();

    // Render file "welcome.html" dengan data title
    view(&req, "welcome.html", context! {
        title => "Selamat Datang di RustBasic",
        auth_installed => auth_installed,
    })
}

pub async fn dev_info(State(state): State<AppState>, _req: Request) -> impl IntoResponse {
    ResponseHelper::json(serde_json::json!({
        "status": "success",
        "app_name": state.config.app_name,
        "environment": if state.config.app_debug { "development" } else { "production" },
        "timezone": state.config.app_timezone,
        "rate_limit": state.config.app_limit_request
    }))
}
