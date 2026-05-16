use rustbasic_core::tower_http::services::ServeDir;
use rustbasic_core::dotenvy::dotenv;
use rustbasic_core::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env & Inisialisasi Logger (Terminal + File)
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menjalankan server.");
    let _guard = rustbasic_core::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 2.1 Cek Command CLI (migrate, seed, storage:link)
    let args: Vec<String> = std::env::args().collect();
    if rustbasic_cli::handle::<rustbasic::migrations::Migrator>(&cfg, &args).await {
        return;
    }

    // 3. Setup Database & Sea-ORM
    let db = rustbasic_core::database::connect(&cfg).await;
    // Migrasi TIDAK dijalankan otomatis saat serve.
    // Gunakan 'rustbasic migrate' untuk menjalankan migrasi secara manual.
    
    // 4. Inisialisasi Session Store
    rustbasic_core::session::init_sessions(&cfg).await;
    let session_store = rustbasic_core::session::setup_session(&cfg).await;

    // 5. Bangun Router Aplikasi
    let api_router = rustbasic::routes::api::router()
        .layer(rustbasic::app::http::middleware::cors::cors_middleware());

    let web_router = rustbasic::routes::web::router()
        .layer(rustbasic_core::axum::middleware::from_fn(rustbasic::app::http::middleware::csrf::csrf_middleware));

    let app_router: rustbasic_core::axum::Router<rustbasic_core::server::AppState> = rustbasic_core::axum::Router::new()
        .nest("/api", api_router)
        .merge(web_router);

    // 6. Setup Statics & Jalankan Server
    let static_files = ServeDir::new("public");
    rustbasic_core::server::start_server(cfg, session_store, static_files, db, app_router).await;
}
