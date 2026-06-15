use rustbasic_core::dotenvy::dotenv;
use rustbasic_core::Config;

fn main() {
    rustbasic_core::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}

async fn run() {
    // 1. Muat file .env & Inisialisasi Logger (Terminal + File)
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menjalankan server.");
    let _guard = rustbasic_core::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 2.0 Otomatis jalankan make_auth jika breeze ditambahkan (hanya di mode development/debug)
    #[cfg(all(breeze, debug_assertions))]
    {
        if std::path::Path::new("Cargo.toml").exists() && !std::path::Path::new("src/routes/auth.rs").exists() {
            rustbasic_breeze::make_auth().await;
        }
    }
    // 2.1 Cek Command CLI (migrate, seed, storage:link)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && rustbasic::config::cli::handle(&args, &cfg).await {
        return;
    }

    // 3. Setup Database
    let db = rustbasic_core::database::connect(&cfg).await;

    // 4. Inisialisasi Session Store
    rustbasic_core::session::init_sessions(&cfg).await;
    let session_store = rustbasic_core::session::setup_session(&cfg).await;

    // 5. Bangun Router Aplikasi
    let app_router = rustbasic::routes::build_router();

    // Inject embedded files
    rustbasic_core::view::set_embedded_templates(rustbasic::config::app::EmbeddedTemplates::get);
    rustbasic_core::server::set_embedded_public(rustbasic::config::app::EmbeddedPublic::get);

    // 6. Jalankan Server
    rustbasic_core::server::start_server(cfg, session_store, db, app_router).await;
}
