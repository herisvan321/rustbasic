mod app;
mod routes;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::Router;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Serve static files from public/ directory
    let static_files = ServeDir::new("public");

    // Build our application with routes
    let app = Router::new()
        .merge(routes::web::router())
        .fallback_service(static_files);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
