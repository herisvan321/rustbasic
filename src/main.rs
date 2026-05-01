/* ---------------------------------------------------------
 * 📑 LABEL: ENTRY POINT UTAMA (main.rs)
 * File ini mengatur inisialisasi server dan file statis.
 * --------------------------------------------------------- */

mod app;
mod routes;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::Router;

#[tokio::main]
async fn main() {
    // 1. Inisialisasi Log (untuk melihat aktivitas server)
    tracing_subscriber::fmt::init();

    // 2. Setup Folder Publik (untuk CSS/JS/Gambar)
    let static_files = ServeDir::new("public");

    // 3. Gabungkan Rute dan Jalankan Server
    let app = Router::new()
        .merge(routes::web::router())
        .fallback_service(static_files);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server berjalan di: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
