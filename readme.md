# 🚀 RustBasic (Axum SPA - Version 2026)

Aplikasi web modern berbasis Rust dengan arsitektur **Laravel-inspired**. Dirancang untuk performa maksimal, keamanan tinggi, dan pengalaman pengembang yang luar biasa.

---

## 💎 Fitur Unggulan
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **🎨 Premium Splitscreen UI**: Desain layar terbagi yang modern dan mewah tanpa kartu (*cardless*).
- **📊 Premium Dashboard**: Panel kendali modern dengan statistik real-time dan navigasi sisi kiri yang elegan.
- **🐞 Smart Error Reporting**: Halaman debug detail saat pengembangan (Stack Trace, Template Info) dan halaman minimalis saat produksi.
- **🗄️ Multi-Database Support**: Dukungan native untuk **SQLite** dan **MySQL** melalui SQLx & Sea-ORM.
- **🔑 Session RustBasic**: Sistem session yang aman dan terenkripsi disimpan di database (RustBasicSessionStore).
- **🛡️ CSRF & Security Ready**: Proteksi CSRF terintegrasi dan Security Headers (CSP) otomatis.
- **🎨 Modern Monolith SPA**: Pengalaman Single Page Application (SPA) tanpa reload menggunakan HTMX dan Alpine.js.
- **📂 Ultra-Clean Architecture**: `main.rs` yang sangat minimalis (30 baris) dengan konfigurasi modular.

---

## 🚀 Development

Untuk mempermudah pengembangan, Anda dapat menggunakan fitur **Auto-Reload** (aplikasi otomatis restart saat ada perubahan file) dan **Port Cleaner** (otomatis mematikan proses lama yang menyangkut di port).

### 1. Instalasi Tool (Sekali saja)
Pastikan Anda memiliki `cargo-watch` terinstal di sistem Anda:
```bash
cargo install cargo-watch
```

### 2. Menjalankan Aplikasi dengan Auto-Reload
Kini Anda bisa menggunakan perintah singkat berikut (mirip `php rustbasic serve`):
```bash
cargo serve
```

*Perintah ini secara otomatis menjalankan `cargo watch -x run -w src -w resources`.*

### 3. Membuat Model & Migration
Sama seperti Laravel, Anda dapat membuat model dan migration baru berbasis Rust (Sea-ORM) dengan mudah:
```bash
cargo rustbasic make:model Product -m
```

### 4. Menjalankan Migrasi
Migrasi sekarang bersifat manual untuk keamanan data:
```bash
cargo rustbasic migrate
```

---

## 🛠️ Tech Stack

## 📂 Struktur Proyek Terbaru
```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, JS, Gambar)
├── resources/
│   └── views/            # Template HTML (Minijinja)
├── src/
│   ├── main.rs           # Entry point (Ultra-Clean)
│   ├── app/              # Core Application Logic
│   ├── config/           # Modular Configuration (DB, Session, Server, Log)
│   ├── database/         # Connections & Session Store
│   └── routes/           # Web Routes
└── .env                  # Environment Variables
```

---

## 🚀 Cara Menjalankan

### 1. Persiapan Lingkungan
Salin file `.env.example` ke `.env` dan sesuaikan pengaturan Anda:
```bash
cp .env.example .env
```

### 2. Jalankan Aplikasi (Mode Pengembangan)
Gunakan perintah singkat berikut untuk fitur Auto-Reload dan Manual Migration dan Port Cleaner:
```bash
cargo serve
```

Atau perintah standar Rust:
```bash
cargo run
```

Akses aplikasi di:
👉 **[http://localhost:4000](http://localhost:4000)**

---

## 📝 Tips Pengembangan
- **Tidy Terminal**: Log query database telah difilter agar terminal tetap bersih dan fokus pada log aplikasi.
- **Debug Mode**: Aktifkan `APP_DEBUG=true` di `.env` untuk mendapatkan visualisasi error yang mendetail selama pengembangan.
- **Splitscreen UI**: Gunakan utility class di `style.css` untuk membangun halaman baru dengan tema splitscreen yang konsisten.

---
*Dibuat dengan ❤️ untuk ekosistem Rust. Arsitektur Bersih, Desain Premium, Kecepatan Cahaya.*
