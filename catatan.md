# 📘 Catatan Dokumentasi RustBasic

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Laravel).

---

## 📂 1. Struktur Folder (Struktur Modular & Ultra-Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, JS, Gambar)
├── resources/
│   └── views/            # Template HTML (Minijinja)
├── src/
│   ├── main.rs           # Entry point (Ultra-Clean, hanya 30 baris)
│   ├── app/              # Folder Inti Aplikasi
│   │   ├── http/         # Logic HTTP (Controllers, Middleware)
│   │   ├── providers/    # Service Providers (Database, View)
│   │   └── mod.rs        # Helper global (seperti fungsi render)
│   ├── config/           # Pusat Konfigurasi & Inisialisasi
│   │   ├── app.rs        # Struct Config & Load .env
│   │   ├── database.rs   # Setup DB (MySQL/SQLite) & Migrasi
│   │   ├── session.rs    # Setup Session Store
│   │   ├── server.rs     # Setup Router & Run Server
│   │   ├── logger.rs     # Setup Logging & Startup Banner
│   │   ├── requests.rs   # Request Helper (Laravel Style)
│   │   ├── responses.rs  # Response Helper (Laravel Style)
│   │   └── mod.rs        # Re-export seluruh konfigurasi
│   ├── database/         # Koneksi DB & RustBasicSessionStore
│   └── routes/           # Pengaturan rute (web.rs, mod.rs)
└── .env                  # Pengaturan environment (Port, DB, App Key)
```

---

## ⚙️ 2. Konfigurasi (.env)

Gunakann file `.env` untuk mengatur perilaku aplikasi:
- `APP_NAME`: Nama aplikasi Anda.
- `APP_KEY`: Kunci enkripsi (wajib diawali `base64:`).
- `DB_CONNECTION`: `sqlite` atau `mysql`.
- `SESSION_DRIVER`: `database` atau `file`.
- `APP_DEBUG`: `true` untuk mode pengembang (detil error), `false` untuk produksi.

---

## 🗄️ 3. Database Multi-Driver & Session

### Multi-Database Support
Aplikasi kini mendukung **SQLite** dan **MySQL** secara native melalui `sqlx::AnyPool`. Konfigurasi otomatis berubah hanya dengan mengganti variabel di `.env`.

### RustBasicSessionStore
Session manager khusus yang menyimpan data secara terenkripsi di database. Nama modul telah diperbarui menjadi `RustBasicSessionStore` untuk mencerminkan identitas framework.

---

## 🎨 4. Frontend & UI (Premium Splitscreen)

Desain UI telah ditingkatkan ke level premium tanpa menggunakan kartu (*cardless*):
- **Splitscreen Layout**: Tampilan layar terbagi yang modern untuk halaman login, daftar, dan dashboard.
- **Visual Excellence**: Menggunakan gradasi jernih, tipografi modern (Inter), dan mikro-animasi.
- **SPA Experience**: Navigasi instan tanpa reload halaman menggunakan **HTMX** dan **Alpine.js**.

---

## 📥 5. Request & Response (Laravel Style)

Kini helper Request & Response berada di dalam folder `config` untuk akses yang lebih terpusat:

### Menggunakan Request
```rust
pub async fn controller(req: Request) -> impl IntoResponse {
    let name = req.input_as_str("name").unwrap_or("Tamu");
    let all = req.all();
}
```

### Menggunakan Response
```rust
ResponseHelper::json(data);
ResponseHelper::view(html);
ResponseHelper::redirect_with_success("/home", "Berhasil!", req.session);
```

---

## 🛡️ 6. Keamanan & Performa Terminal

### Keamanan Terintegrasi
- **CSRF Protection**: Otomatis memvalidasi token pada request mutasi.
- **CSP & Security Headers**: Terkonfigurasi untuk memblokir script inline berbahaya dan clickjacking.

### Terminal Output (Tidy Logs)
Terminal telah dibersihkan dari log query SQL yang berulang-ulang. Hanya log penting (Error, Warn, App Debug) yang ditampilkan. Dilengkapi dengan **Startup Banner** ASCII saat aplikasi dijalankan.

---

*Dokumentasi ini diperbarui Mei 2026 mencerminkan arsitektur Modular, Dukungan MySQL, dan Desain Premium Splitscreen.*
