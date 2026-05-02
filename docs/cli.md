# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic. Alat ini dirancang untuk mempercepat alur kerja pengembangan Anda.

## 🚀 Cara Menjalankan
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah yang diinginkan:

```bash
cargo rustbasic <perintah> [argumen]
```

---

## 📂 1. Generator Komponen (Scaffolding)

### `make:model`
Membuat file Entity Sea-ORM baru di folder `src/app/models/`.
- **Argumen**: `<NamaModel>`
- **Opsi**: `-m` (Otomatis buat file migrasi terkait)
- **Contoh**: `cargo rustbasic make:model Product -m`

### `make:migration`
Membuat file migrasi Rust (Sea-ORM) baru dengan timestamp otomatis di `database/migrations/`.
- **Argumen**: `<NamaMigration>`
- **Contoh**: `cargo rustbasic make:migration add_price_to_products`

### `make:controller`
Membuat Controller Axum baru di `src/app/http/controllers/` dan otomatis mendaftarkannya di `mod.rs`.
- **Argumen**: `<NamaController>`
- **Contoh**: `cargo rustbasic make:controller Product`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/` dan otomatis mendaftarkannya di `mod.rs`.
- **Argumen**: `<NamaMiddleware>`
- **Contoh**: `cargo rustbasic make:middleware CheckRole`

---

## 🗄️ 2. Database & Cache

### `migrate`
Menjalankan seluruh file migrasi yang ada ke database (SQLite/MySQL).
- **Contoh**: `cargo rustbasic migrate`

### `cache:clear`
Membersihkan sistem secara menyeluruh:
1. Mengosongkan file log di `storage/logs/` (truncate).
2. Menghapus seluruh data sesi di database.
- **Contoh**: `cargo rustbasic cache:clear`

---

## 🔍 3. Monitoring & Security

### `route:list`
Menampilkan tabel daftar rute yang terdaftar di aplikasi (Method, Path, dan Handler).
- **Contoh**: `cargo rustbasic route:list`

### `check:security`
Melakukan audit keamanan aplikasi:
- Cek keberadaan Middleware CSRF.
- Cek library hashing password (Bcrypt).
- Proteksi SQL Injection & XSS.
- Audit kerentanan dependency via `cargo audit`.
- **Contoh**: `cargo rustbasic check:security`

### `check:update`
Menghubungi crates.io untuk mengecek apakah ada versi terbaru dari paket (dependencies) yang Anda gunakan.
- **Contoh**: `cargo rustbasic check:update`

---

## 🚀 4. Build Manager

### `build`
Menjalankan menu interaktif untuk membangun (compile) aplikasi ke berbagai sistem operasi:
- **Target OS**: Native, Windows, Linux, atau macOS.
- **Mode**: Development atau Production (Release).
- **Fitur**: Mendukung `cargo-zigbuild` untuk cross-compilation yang lebih mudah.
- **Contoh**: `cargo rustbasic build`

---

_Dokumentasi ini diekstrak langsung dari implementasi teknis di `src/config/cli.rs`._
