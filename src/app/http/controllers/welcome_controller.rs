use crate::app::view;
use crate::app::http::requests::Request;
use crate::app::http::responses::ResponseHelper;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn index(req: Request) -> impl IntoResponse {
    // Render file "welcome.html" dengan data title
    view(&req, "welcome.html", context! {
        title => "Selamat Datang di RustBasic",
    })
}

/// Contoh penggunaan Request & Response ala Laravel
pub async fn test_request(req: Request) -> impl IntoResponse {
    let name = req.input("name").unwrap_or_else(|| "Tamu".to_string());
    
    ResponseHelper::json(serde_json::json!({
        "message": format!("Halo, {}! Ini adalah data dari Request ala Laravel.", name),
        "method": req.method.to_string(),
        "all_inputs": req.all()
    }))
}
