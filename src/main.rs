use tower_http::services::ServeDir;
use dotenvy::dotenv;
use rustbasic_core::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env & Inisialisasi Logger (Terminal + File)
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menjalankan server.");
    let _guard = rustbasic_core::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 3. Setup Database & Sea-ORM
    let db = rustbasic_core::database::connect(&cfg).await;
    //rustbasic_core::database::run_migrations(&db).await; // <-- Jalankan migrasi otomatis
    
    // 4. Inisialisasi Session Store
    rustbasic_core::session::init_sessions(&cfg).await;
    let session_store = rustbasic_core::session::setup_session(&cfg).await;

    // 5. Bangun Router Aplikasi
    let app_router = rustbasic_core::Router::new()
        .merge(rustbasic::routes::web::router())
        .layer(rustbasic_core::middleware::from_fn(rustbasic::app::http::middleware::csrf::csrf_middleware))
        .layer(rustbasic_core::middleware::from_fn(rustbasic::app::http::middleware::security_headers::security_headers_middleware))
        .layer(rustbasic_core::middleware::from_fn(rustbasic::app::http::middleware::logging::logging_middleware));

    // 6. Setup Statics & Jalankan Server
    let static_files = ServeDir::new("public");
    rustbasic_core::server::start_server(cfg, session_store, static_files, db, app_router).await;
}
