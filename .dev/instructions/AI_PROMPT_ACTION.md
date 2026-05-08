# 🦾 AI Action Center: RustBasic Framework

prompt
pastikan perpindah antar halaman tanpa reload/refresh menggunakan htmx, saya ingin berpindah antar halaman tanpa ada reload/refresh, 

---

## 📂 Struktur Folder (Modular & Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:
csrf protection pada htmx dengan menggunakan cargo rustbasic, untuk input type=csrf wajib diubah menjadi token htmx

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar)
├── src/resources/
│   └── css/              # CSS files
│   └── js/               # JS files
│   └── views/            # Template .rb.html (HTML + Minijinja)
│       └── layouts/      # Layout utama (app.rb.html)
├── src/
│   ├── main.rs           # Entry point (Strict Config & Mandatory .env)
│   ├── app/              # Folder Inti Aplikasi (Controllers, Models, Middleware)
│   ├── config/           # Pusat Konfigurasi (Server, Session, View Engine)
│   └── routes/           # Pengaturan rute
├── storage/              # Penyimpanan File & Log
├── docs/                 # Dokumentasi Lengkap
└── .env                  # Environment Variables (Wajib Ada)
```

---

## 🛡️ Standar Penulisan Template (WAJIB)
AI harus selalu menggunakan standar ini saat memodifikasi tampilan:
1. **Ekstensi**: Selalu gunakan `.rb.html`.
2. **Sintaks**: Gunakan tag HTML standar dan tag Minijinja (`{{ variable }}`, `{% block content %}`). Sistem RSX telah **dihapus**.
3. **Modern UI**: Wajib menggunakan estetika modern (Split-Screen, Glassmorphism, CSS Mesh Gradients) seperti pada modul Auth.
4. **Hybrid Embedding**: Memahami bahwa template di-embed ke binary saat *Release* (via `rust-embed`) tapi tetap bisa di-edit secara live saat *Debug*.
5. **Source Protection**: Output HTML otomatis diminifikasi oleh server.

---

# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) lengkap untuk framework RustBasic.

## 🚀 Cara Menjalankan
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah yang diinginkan:

```bash
cargo rustbasic <perintah> [argumen]
```

Atau gunakan alias langsung jika sudah dikonfigurasi:
```bash
rb <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

### `cargo rustbasic serve`
Menjalankan server dalam mode pengembangan:
- **Template Rendering**: Mengolah `.rb.html` dengan Minijinja.
- **Auto-Watch**: Memantau perubahan pada `src/`, `src/resources/` (template), dan file `.env`.
- **Live Reload**: Otomatis merestart server dan me-refresh browser.
- **Contoh**: `cargo rustbasic serve`

---

## 📂 1. Generator (Scaffolding)

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

## 🔐 2. Authentication Scaffolding

### `auth` / `make:auth`
Membangun sistem autentikasi lengkap secara otomatis.
- **Fitur**: Login, Register, Forgot Password, Reset, dan Dashboard premium.
- **UI**: Desain Modern Split-Screen dengan Floating Toast Notifications.
- **Logic**: Mengintegrasikan Sea-ORM, Bcrypt, dan validasi secara otomatis.
- **Contoh**: `cargo rustbasic auth`

### `auth back` / `auth:back`
Menghapus seluruh sistem autentikasi dan mengembalikan project ke kondisi bersih.
- **Contoh**: `cargo rustbasic auth back`

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan seluruh file migrasi yang ada ke database (SQLite/MySQL).
- **Contoh**: `cargo rustbasic migrate`

### `migrate:refresh`
Melakukan rollback seluruh migrasi dan menjalankannya ulang dari awal. Sangat berguna saat fase development untuk mereset struktur tabel.
- **Contoh**: `cargo rustbasic migrate:refresh`

### `migrate:back` / `migrate:rollback`
Membatalkan migrasi terakhir (rollback 1 step).
- **Contoh**: `cargo rustbasic migrate:back`

### `cache:clear`
Membersihkan sistem secara menyeluruh (log dan data sesi).
- **Contoh**: `cargo rustbasic cache:clear`
    
### `key:generate`
Membuat kunci aplikasi baru (`APP_KEY`) yang aman.
- **Contoh**: `cargo rustbasic key:generate`

---

## 🔍 4. Monitoring & Security

### `route:list`
Menampilkan tabel daftar rute yang terdaftar di aplikasi (Method, Path, dan Handler).

### `check:security`
Melakukan audit keamanan aplikasi (CSRF, Bcrypt, XSS protection).

### `check:update`
Mengecek pembaruan dependencies di crates.io.

---

## 🚀 5. Build Manager

### `build`
Menu interaktif untuk kompilasi aplikasi ke berbagai sistem operasi (Cross-build).
- **Contoh**: `cargo rustbasic build`

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic Framework._
