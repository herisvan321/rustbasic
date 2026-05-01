# 🚀 RustBasic (Axum SPA)

Aplikasi web modern berbasis Rust dengan arsitektur **Laravel-inspired**. Dirancang untuk performa maksimal, keamanan tinggi, dan kemudahan pengembangan.

---

## 💎 Fitur Unggulan
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum dan Tokio.
- **🗄️ Dual-Database Ready**: Dukungan otomatis untuk **SQLite** dan **MySQL** menggunakan **Sea-ORM**.
- **🔑 Session ala Laravel**: Sistem session dengan skema tabel database standar Laravel (IP Address, User Agent, dll).
- **📥 Request & Response Helpers**: Gunakan `$request->input()` dan `Response::json()` layaknya di Laravel.
- **⚙️ Config via .env**: Pengaturan aplikasi terpusat dalam file `.env` (Port, DB, App Key).
- **🎨 Premium UI**: Desain modern menggunakan Vanilla CSS, HTMX, dan Alpine.js.
- **📂 Modular Structure**: Folder terorganisir rapi (`config/`, `database/`, `app/`, `routes/`).

---

## 📂 Struktur Proyek Terbaru
```text
rustbasic/
├── database/             # Tempat penyimpanan SQLite (.sqlite) & SQL Schema
├── public/               # File statis (CSS, JS, Gambar)
├── resources/            # Template HTML (Minijinja)
├── src/
│   ├── main.rs           # Entry Point & Middleware Session
│   ├── config/           # Loader Konfigurasi (.env)
│   ├── database/         # Koneksi DB & Custom Session Store
│   ├── routes/           # Web & API Routing
│   └── app/              # Folder Inti Aplikasi
│       └── http/         # Logic HTTP (Controllers, Requests, Responses)
│           ├── controllers/
│           ├── requests/
│           └── responses/
└── .env                  # File Pengaturan Utama (Rahasia)
```

---

## 🚀 Cara Menjalankan

### 1. Persiapan Lingkungan
Salin file `.env.example` ke `.env` (jika ada) atau pastikan file `.env` Anda sudah benar:
```env
APP_NAME=RustBasic
APP_PORT=4000
DB_CONNECTION=sqlite
DB_DATABASE=rustbasic
SESSION_DRIVER=database
```

### 2. Jalankan Aplikasi
```bash
cargo run
```
Setelah berjalan, akses di:
👉 **[http://localhost:4000](http://localhost:4000)**

---

## 📝 Contoh Penggunaan Laravel Style

### Request & Response
```rust
pub async fn store(req: Request) -> impl IntoResponse {
    let name = req.input("name").unwrap_or_default();
    Response::json(json!({ "status": "success", "user": name }))
}
```

---

## 📝 Konfigurasi Database & Session
- **SQLite**: Secara otomatis disimpan di dalam folder `database/`.
- **MySQL**: Cukup ubah `DB_CONNECTION=mysql` dan isi kredensial di `.env`.
- **Session**: Tabel `sessions` akan dibuat otomatis di database dengan kolom audit lengkap (ID, Payload, Last Activity).

---

## 🛠️ Troubleshooting
- **Error Database?** Pastikan folder `database/` ada dan memiliki izin tulis.
- **Port Bentrok?** Ubah `APP_PORT` di file `.env`.
- **Cargo Error?** Pastikan Anda menggunakan versi Rust terbaru (`rustup update`).

---
*Dibuat dengan ❤️ untuk komunitas Rust Indonesia. Arsitektur Bersih, Kode Rapi.*
