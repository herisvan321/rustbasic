# 🔑 Paket Autentikasi JWT (rustbasic-jwt)

Paket `rustbasic-jwt` menyediakan sistem autentikasi stateless berbasis JSON Web Token (JWT) yang sangat aman dan berkinerja tinggi untuk framework RustBasic. Paket ini mendukung pembuatan token, pemutakhiran (*refreshing*), pembatalan (*blacklisting*), dan integrasi middleware otorisasi.

---

## ⚙️ 1. Konfigurasi Awal (.env)

Konfigurasi token disimpan di dalam file `.env` proyek Anda. Saat pertama kali paket ini dipasang, file `.env` akan diisi secara otomatis dengan nilai default:

```env
# --- JWT CONFIG ---
JWT_SECRET=rahasia_kunci_unik_anda_di_sini
JWT_TTL=60               # Masa berlaku token dalam menit (Default: 60)
JWT_REFRESH_TTL=20160    # Masa berlaku refresh token dalam menit (Default: 14 hari)
JWT_ALGO=HS256           # Algoritma penandatanganan: HS256, HS384, atau HS512
```

---

## 🛠️ 2. Penggunaan Layanan `JwtService`

`JwtService` adalah layanan utama untuk mengelola siklus hidup token.

```rust
use rustbasic_jwt::JwtService;
use rustbasic_core::serde_json::json;

// 1. Inisialisasi Layanan JWT (menyertakan koneksi database untuk fitur blacklist)
let jwt_service = JwtService::new().with_db(db_pool);

// 2. Pembuatan Token (Generate Token)
let user_id = "123".to_string();
let user_data = json!({
    "name": "Hendra",
    "email": "hendra@example.com",
    "role": "admin"
});

match jwt_service.generate_token(user_id, user_data) {
    Ok(token) => println!("Token JWT: {}", token),
    Err(e) => eprintln!("Gagal membuat token: {}", e),
}
```

### Metode Lainnya pada `JwtService`:
*   **`validate_token(&self, token: &str) -> Result<Claims, String>`**: Memvalidasi integritas tanda tangan dan masa kedaluwarsa token.
*   **`invalidate_token(&self, token: &str) -> Result<(), String>`**: Memasukkan token secara permanen ke daftar hitam (*blacklist*) database.
*   **`refresh_token(&self, token: &str) -> Result<String, String>`**: Membatalkan token lama dan menghasilkan token baru dengan masa berlaku segar.

---

## 🛡️ 3. Mengamankan Rute dengan Middleware

Paket ini menyediakan middleware bawaan bernama `jwt_auth_middleware` yang mengekstrak token dari header `Authorization: Bearer <token>`, memvalidasinya, dan mengikat data pengguna ke dalam Request Scope.

### Contoh Registrasi Middleware pada Router Backend:

```rust
use rustbasic_core::{Router, get, post};
use rustbasic_jwt::jwt_auth_middleware;

pub fn routes() -> Router {
    Router::new()
        // Rute publik (tidak memerlukan autentikasi)
        .route("/login", post(login_controller))
        // Rute terproteksi (menggunakan middleware JWT)
        .route("/profile", get(profile_controller).middleware(jwt_auth_middleware))
}
```

---

## 👤 4. Mengakses Informasi Pengguna Aktif

Di dalam controller/handler, Anda dapat mengambil data klaim pengguna aktif yang terikat pada *request lifetime* saat ini secara aman menggunakan `get_current_user()`:

```rust
use rustbasic_core::{Response, ResponseHelper};
use rustbasic_jwt::get_current_user;

pub async fn profile_controller() -> Response {
    if let Some(claims) = get_current_user() {
        // Mengakses sub (user_id) dan payload kustom
        let user_id = claims.sub;
        let user_data = claims.user_data;

        ResponseHelper::json(json!({
            "status": "success",
            "user_id": user_id,
            "profile": user_data
        }))
    } else {
        ResponseHelper::error("Unauthorized")
    }
}
```

---

## 🗄️ 5. Skema Database Blacklist

Untuk melacak token yang telah dikeluarkan/dibatalkan (*logout*), tabel migrasi `jwt_blacklists` dibuat secara otomatis di bawah folder `database/migrations/`:

```rust
use rustbasic_core::{Schema, SchemaManager, MigrationTrait, DbErr};
use rustbasic_core::async_trait;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    fn name(&self) -> &str { "create_jwt_blacklists_table" }

    async fn up<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::create(manager, "jwt_blacklists", |table| {
            table.string("jti").unique().not_null(); // Token Identifier unik
            table.big_integer("exp").not_null();     // Waktu kedaluwarsa token
        }).await?;
        Ok(())
    }

    async fn down<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::drop(manager, "jwt_blacklists").await?;
        Ok(())
    }
}
```
