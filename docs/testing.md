# 🧪 Panduan Testing (Unit, Feature, & Database Test)

## 📝 Kata Pengantar

Pengujian kode (*testing*) adalah bagian krusial dalam siklus pengembangan perangkat lunak untuk menjamin keandalan, stabilitas, dan keamanan aplikasi Anda. Di dalam framework **RustBasic**, kami menyediakan sistem pengujian modular yang didesain agar intuitif, modern, dan mudah dipahami (*in-memory testing*).

Anda dapat membuat pengujian tingkat rendah seperti pengujian logika fungsi terisolasi (*Unit Testing*), pengujian alur request-response HTTP in-memory secara lengkap (*Feature Testing*), hingga pengujian yang berinteraksi langsung dengan database (*Database Integration Testing*). Dokumentasi ini dirancang untuk memandu Anda dari level pemula (cara membuat dan menjalankan test sederhana) hingga mahir (melakukan asersi database dan HTTP).

---

## 🛠️ Script Contoh

Berikut adalah contoh implementasi lengkap untuk pengujian unit, pengujian feature, dan pengujian database.

### A. Contoh Unit Test Biasa (`tests/unit_math_test.rs`)
Unit test digunakan untuk menguji logika bisnis kecil yang terisolasi tanpa memerlukan inisialisasi server, router, atau database.

```rust
/* ---------------------------------------------------------
 * 🧪 UNIT TEST: Math (tests/unit_math_test.rs)
 * --------------------------------------------------------- */

#[test]
fn test_math_logic() {
    // 1. Arrange (Persiapkan data)
    let expected = 42;
    
    // 2. Act (Eksekusi logika)
    let actual = 40 + 2;
    
    // 3. Assert (Asersi hasil akhir)
    assert_eq!(expected, actual, "Fungsi kalkulasi matematika tidak sesuai");
}
```

### B. Contoh Feature Test HTTP (`tests/feature_welcome_test.rs`)
Feature test digunakan untuk menguji fungsionalitas HTTP request-response secara lengkap secara *in-memory* tanpa mengonsumsi port TCP jaringan (sangat cepat!).

```rust
/* ---------------------------------------------------------
 * 🧪 FEATURE TEST: Welcome (tests/feature_welcome_test.rs)
 * --------------------------------------------------------- */

use rustbasic_core::testing::TestClient;
use rustbasic_core::Config;

#[tokio::test]
async fn test_welcome_page() {
    // 1. Muat konfigurasi .env
    let cfg = Config::load();
    
    // 2. Bangun router aplikasi utama
    let router = rustbasic::routes::build_router();
    
    // 3. Inisialisasi TestClient in-memory
    let client = TestClient::new(cfg, router).await;
    
    // 4. Jalankan simulasi request HTTP GET ke '/'
    let response = client.get("/").await;
    
    // 5. Verifikasi response menggunakan helper asersi bawaan
    response.assert_status(200);
    response.assert_see("Welcome");
}
```

### C. Contoh Unit Test dengan Database (`tests/unit_database_test.rs`)
Jika logika atau model aplikasi Anda berinteraksi dengan database, gunakan pendekatan berikut untuk menginisialisasi skema database secara otomatis (*auto-migrate*) dan melakukan operasi CRUD data uji coba secara aman terisolasi.

```rust
/* ---------------------------------------------------------
 * 🧪 UNIT TEST: Database (tests/unit_database_test.rs)
 * --------------------------------------------------------- */

use rustbasic_core::{Config, MigratorTrait};
use rustbasic_core::database::DB;
use rustbasic_core::serde_json::json;
use rustbasic::app::models::users;

#[tokio::test]
async fn test_database_insert_and_query() {
    // 1. Muat konfigurasi .env
    let cfg = Config::load();
    
    // 2. Hubungkan ke database lokal
    let db = rustbasic_core::database::connect(&cfg).await;
    
    // 2.5 Jalankan migrasi secara programatis untuk membuat tabel jika belum ada
    let _ = rustbasic::migrations::Migrator::up(&db, None).await;
    
    // 3. Bersihkan data user test sebelumnya (jika ada) demi isolasi test
    let _ = DB::table(&db, "users")
        .where_("email", "john_test@example.com")
        .delete()
        .await;

    // 4. Arrange: Siapkan data user baru dalam format JSON
    let user_data = json!({
        "name": "John Test",
        "email": "john_test@example.com",
        "password": "hashedpassword123"
    });

    // 5. Act: Insert data menggunakan fungsi create bawaan model ORM
    let created_user = users::Model::create(&db, user_data).await
        .expect("Gagal membuat user baru di database");

    // 6. Assert: Periksa nilai kembalian model::create
    assert_eq!(created_user.name, "John Test");
    assert_eq!(created_user.email, "john_test@example.com");

    // 7. Act: Tarik kembali data dari database untuk membuktikan persistensi data
    let retrieved_user = DB::table(&db, "users")
        .where_("email", "john_test@example.com")
        .first::<users::Model>()
        .await
        .expect("Query ke database gagal")
        .expect("User tidak ditemukan di database");

    // 8. Assert: Pastikan data yang diambil sama dengan data awal yang dimasukkan
    assert_eq!(retrieved_user.name, "John Test");
    assert_eq!(retrieved_user.email, "john_test@example.com");

    // 9. Clean up: Hapus data setelah test selesai agar database tetap bersih
    let _ = DB::table(&db, "users")
        .where_("email", "john_test@example.com")
        .delete()
        .await;
}
```

### D. Contoh Feature Test untuk Autentikasi (Register & Login) (`tests/feature_auth_test.rs`)
Pengujian fitur autentikasi menggabungkan integrasi HTTP *in-memory* menggunakan `TestClient` dan interaksi database menggunakan model `User` untuk memastikan fungsionalitas login dan registrasi dari scaffolding **RustBasic Breeze** berjalan dengan aman dan benar.

```rust
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
    
    // Hubungkan ke database
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

    // 3. Register embedded files (diperlukan untuk in-memory template rendering jika ada redirect)
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

    // 7. Clean up setelah test selesai agar database tetap bersih
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

    // 2. Kirim request POST ke '/register' dengan data tidak valid (nama terlalu pendek, format email salah, password terlalu pendek)
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
```

---

## 🔄 Perbandingan Skenario Pengujian

Berikut adalah panduan perbandingan skenario pengujian yang tepat untuk membantu Anda memilih jenis test yang sesuai kebutuhan aplikasi:

### 1. Unit Test Logika
Skenario terbaik: Pengujian helper kalkulasi, validator kustom, pemformatan string, parser logika, dll.
- **Karakteristik**: Berjalan sangat cepat (mikrodetik), tanpa koneksi database atau jaringan, tanpa router HTTP.
- **Sintaksis**:
  ```rust
  #[test]
  fn test_logika() {
      assert_eq!(2 + 2, 4);
  }
  ```

### 2. Feature/Endpoint Test
Skenario terbaik: Pengujian endpoint API, routing controller, autentikasi middleware, respon Inertia view, dll.
- **Karakteristik**: Simulasi request HTTP in-memory menggunakan `TestClient`, berjalan tanpa binding socket network.
- **Sintaksis**:
  ```rust
  #[tokio::test]
  async fn test_halaman() {
      let response = client.get("/about").await;
      response.assert_status(200);
  }
  ```

### 3. Database Test
Skenario terbaik: Pengujian query builder, create/update/delete model, migrasi otomatis, integritas relasi tabel, dll.
- **Karakteristik**: Memerlukan pool database aktif, menjalankan migrasi secara otomatis (`Migrator::up`), dan memerlukan cleanup data uji coba.
- **Sintaksis**:
  ```rust
  let _ = rustbasic::migrations::Migrator::up(&db, None).await;
  let user = users::Model::create(&db, data).await;
  ```

---

## 📊 Tabel Ringkasan

Tabel berikut meringkas berbagai metode asersi (*assertion helpers*) yang tersedia pada `TestResponse` untuk menyederhanakan asersi pengujian Anda:

| Metode Asersi | Deskripsi Fungsi | Contoh Pemakaian |
|---|---|---|
| `.status()` | Mengambil kode status HTTP response dalam tipe `u16`. | `assert_eq!(response.status(), 200);` |
| `.text()` | Mengambil isi response body dalam format `String`. | `let body = response.text();` |
| `.json::<T>()` | Mengurai body JSON menjadi struct Rust yang mengimplementasikan `Deserialize`. | `let data: User = response.json().unwrap();` |
| `.assert_status(code)` | Memvalidasi apakah status HTTP response sama dengan `code` (memicu panic jika berbeda). | `response.assert_status(200);` |
| `.assert_see(val)` | Memvalidasi apakah string `val` ada di dalam response body. | `response.assert_see("RustBasic");` |
| `.assert_dont_see(val)` | Memvalidasi apakah string `val` **tidak** ada di dalam response body. | `response.assert_dont_see("Error 500");` |

---

## 🏁 Penutup

Dengan hadirnya sistem testing *in-memory* yang powerful dan integrasi CLI generator di RustBasic, Anda dapat memastikan kualitas kode aplikasi Anda tetap terjaga seiring berkembangnya skala proyek. Pengujian dijalankan secara lokal dalam milidetik, tanpa proses binding socket network TCP, menjadikannya ideal untuk otomatisasi CI/CD dan alur TDD (*Test-Driven Development*). Selamat menulis test Anda!
