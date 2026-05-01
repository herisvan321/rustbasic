mod app;
mod routes;
mod config;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::Router;
use dotenvy::dotenv;
use crate::config::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env
    dotenv().ok();

    // 2. Inisialisasi Log (untuk melihat aktivitas server)
    tracing_subscriber::fmt::init();

    // 3. Muat Konfigurasi
    let cfg = Config::load();

    // 4. Setup Folder Publik (untuk CSS/JS/Gambar)
    let static_files = ServeDir::new("public");

    // 5. Gabungkan Rute dan Jalankan Server
    let app = Router::new()
        .merge(routes::web::router())
        .fallback_service(static_files);

    let addr_str = format!("{}:{}", cfg.app_host, cfg.app_port);
    let addr: SocketAddr = addr_str.parse().expect("Alamat server tidak valid");
    
    tracing::info!("{} berjalan di: http://{}", cfg.app_name, addr);
    tracing::info!("APP_KEY berhasil dimuat (Keamanan Aktif)");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
