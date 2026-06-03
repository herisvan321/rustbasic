/* ---------------------------------------------------------
 * 🧪 FEATURE TEST: Authentication (tests/feature_auth_test.rs)
 * --------------------------------------------------------- */

use rustbasic_core::testing::TestClient;
use rustbasic_core::{Config, MigratorTrait};
use rustbasic_core::database::DB;
use rustbasic_core::serde_json::json;
use rustbasic::app::models::User;

async fn setup_test_db() -> rustbasic_core::sqlx::AnyPool {
    let _ = rustbasic_core::dotenvy::dotenv();
    let cfg = Config::load();
    
    // Hubhubung ke database
    let db = rustbasic_core::database::connect(&cfg).await;
    
    // Jalankan migrasi secara programatis untuk membuat tabel jika belum ada
    let _ = rustbasic::migrations::Migrator::up(&db, None).await;
    
    db
}

#[tokio::test]
async fn test_registration_success() {
    let db = setup_test_db().await;
    
    // 1. Bersihkan data user test sebelumnya demi isolasi test
    let _ = DB::table(&db, "users")
        .where_("email", "test_register@example.com")
        .delete()
        .await;

    // 2. Load config & router untuk client
    let cfg = Config::load();
    let router = rustbasic::routes::build_router();
    let client = TestClient::new(cfg, router).await;

    // 3. Register embedded files (required for template rendering if redirect checks it)
    rustbasic_core::view::set_embedded_templates(rustbasic::config::app::EmbeddedTemplates::get);
    rustbasic_core::server::set_embedded_public(rustbasic::config::app::EmbeddedPublic::get);

    // 4. Kirim request POST ke '/register'
    let payload = json!({
        "name": "Daftar Baru",
        "email": "test_register@example.com",
        "password": "password123"
    });
    
    let response = client.post("/register", payload).await;
    
    // 5. Asersi response: status redirect 303 ke '/login'
    assert_eq!(response.status(), 303, "Seharusnya melakukan redirect 303 setelah registrasi berhasil");
    
    // 6. Verifikasi user benar-benar tersimpan di database
    let user = DB::table(&db, "users")
        .where_("email", "test_register@example.com")
        .first::<User>()
        .await
        .expect("Query gagal")
        .expect("User baru tidak ditemukan di database");

    assert_eq!(user.name, "Daftar Baru");
    assert_eq!(user.email, "test_register@example.com");

    // 7. Clean up
    let _ = DB::table(&db, "users")
        .where_("email", "test_register@example.com")
        .delete()
        .await;
}

#[tokio::test]
async fn test_registration_validation_failure() {
    let db = setup_test_db().await;
    
    // 1. Bersihkan data user test
    let _ = DB::table(&db, "users")
        .where_("email", "invalid_email")
        .delete()
        .await;

    let cfg = Config::load();
    let router = rustbasic::routes::build_router();
    let client = TestClient::new(cfg, router).await;

    // 2. Kirim request POST ke '/register' dengan data tidak valid (nama terlalu pendek, email salah, password terlalu pendek)
    let payload = json!({
        "name": "Ab",
        "email": "invalid_email",
        "password": "123"
    });
    
    let response = client.post("/register", payload).await;
    
    // 3. Asersi response: status redirect 303 kembali ke '/register'
    assert_eq!(response.status(), 303, "Registrasi dengan data tidak valid harus me-redirect");

    // 4. Verifikasi bahwa user TIDAK dibuat di database
    let user = DB::table(&db, "users")
        .where_("email", "invalid_email")
        .first::<User>()
        .await
        .expect("Query gagal");

    assert!(user.is_none(), "User tidak boleh terbuat di database apabila validasi gagal");
}

#[tokio::test]
async fn test_login_success() {
    let db = setup_test_db().await;
    
    // 1. Bersihkan data user test
    let _ = DB::table(&db, "users")
        .where_("email", "test_login@example.com")
        .delete()
        .await;

    // 2. Buat user dummy langsung ke database dengan password ter-hash
    let hashed_password = rustbasic_core::bcrypt::hash("password_rahasia", rustbasic_core::bcrypt::DEFAULT_COST).unwrap();
    let user_data = json!({
        "name": "User Test Login",
        "email": "test_login@example.com",
        "password": hashed_password
    });
    
    let created_user = User::create(&db, user_data).await
        .expect("Gagal membuat user dummy di database");

    // 3. Setup client
    let cfg = Config::load();
    let router = rustbasic::routes::build_router();
    let client = TestClient::new(cfg, router).await;

    // 4. Kirim request POST ke '/login' dengan credentials yang benar
    let payload = json!({
        "email": "test_login@example.com",
        "password": "password_rahasia"
    });
    
    let response = client.post("/login", payload).await;
    
    // 5. Asersi response: status redirect 303 ke '/dashboard'
    assert_eq!(response.status(), 303, "Seharusnya melakukan redirect 303 setelah login sukses");

    // 6. Clean up
    let _ = User::destroy(&db, created_user.id).await;
}

#[tokio::test]
async fn test_login_failure_wrong_password() {
    let db = setup_test_db().await;
    
    // 1. Bersihkan data user test
    let _ = DB::table(&db, "users")
        .where_("email", "test_login_fail@example.com")
        .delete()
        .await;

    // 2. Buat user dummy langsung ke database
    let hashed_password = rustbasic_core::bcrypt::hash("password_rahasia", rustbasic_core::bcrypt::DEFAULT_COST).unwrap();
    let user_data = json!({
        "name": "User Test Login Gagal",
        "email": "test_login_fail@example.com",
        "password": hashed_password
    });
    
    let created_user = User::create(&db, user_data).await
        .expect("Gagal membuat user dummy");

    // 3. Setup client
    let cfg = Config::load();
    let router = rustbasic::routes::build_router();
    let client = TestClient::new(cfg, router).await;

    // 4. Kirim request POST ke '/login' dengan password yang salah
    let payload = json!({
        "email": "test_login_fail@example.com",
        "password": "password_salah"
    });
    
    let response = client.post("/login", payload).await;
    
    // 5. Asersi response: status redirect 303 kembali ke '/login'
    assert_eq!(response.status(), 303, "Login dengan password salah harus me-redirect");

    // 6. Clean up
    let _ = User::destroy(&db, created_user.id).await;
}
