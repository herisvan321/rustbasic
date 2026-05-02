# 🚀 RustBasic (Axum SPA - Zero-JS Edition 2026)

Aplikasi web modern berbasis Rust dengan arsitektur **Clean Monolith**. Dirancang untuk performa maksimal, keamanan tinggi, dan pengalaman pengembang yang luar biasa dengan filosofi **Zero-JS**.

---

## 💎 Fitur Unggulan

- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **🎨 Zero-JS UI Architecture**: Pengalaman Single Page Application (SPA) yang sangat ringan menggunakan **HTMX** dan **Pure CSS**. Tidak ada library JavaScript berat (Hapus Alpine.js & Vanilla JS).
- **🧩 Modular Minijinja Macros**: UI yang dibangun dengan komponen reusable yang terbagi secara logis (`forms`, `buttons`, `display`, `overlays`, `feedback`).
- **📅 Carbon-like Time Management**: Penanganan waktu yang mudah dan kuat menggunakan `chrono` & `chrono-humanize`. Mendukung `.diff_for_humans()` dan zona waktu dinamis via `.env`.
- **🔐 Hardened Security**: 
    - **Session-IP Binding**: Sesi dikunci berdasarkan IP Address untuk mencegah hijacking.
    - **Strict Env Enforcement**: Aplikasi tidak akan berjalan tanpa file `.env` yang valid.
    - **CSRF Protection**: Proteksi otomatis pada semua request HTMX.
- **📝 Production-Grade Logging**: Dual-output logging (Terminal berwarna & File bersih di `storage/logs/`).
- **🔘 Smart Overlays**: Modal konfirmasi (seperti Logout) menggunakan teknik **CSS Checkbox Hack** (Zero-JS).
- **🚀 Premium Splitscreen UI**: Desain layar terbagi yang modern dan mewah tanpa kartu (_cardless_).
- **📊 Premium Dashboard**: Panel kendali modern dengan statistik real-time dan navigasi sisi kiri yang elegan.
- **📦 Hidden Assets & Binary Embedding**: File CSS dan JS (HTMX) disembunyikan dari folder publik dan ditanam langsung ke dalam binary aplikasi menggunakan `include_str!` untuk performa maksimal dan keamanan ekstra.

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

_Perintah ini secara otomatis menjalankan `cargo watch` yang juga memantau perubahan pada file `.env`._

---

## 🛠️ Tech Stack & Components

### UI Component Library
Terletak di `resources/views/components/`:
- **`forms.html`**: Input field, checkbox, dll.
- **`buttons.html`**: Tombol aksi, link button, tombol kembali.
- **`display.html`**: Card, Premium Alerts (Floating), Stat Cards.
- **`overlays.html`**: Modal dinamis dan Konfirmasi Logout (Zero-JS).
- **`feedback.html`**: Loading Spinners dan HTMX Indicators.

---

## 📂 Struktur Proyek Terbaru

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar) - ZERO JS
├── resources/
│   ├── css/              # Asset CSS (Hidden/Embedded)
│   ├── js/               # Asset JS (Hidden/Embedded)
│   └── views/            # Template HTML (Minijinja)
│       ├── auth/         # Halaman Login & Register
│       ├── components/   # Modular UI Components
│       ├── errors/       # Template Error (404, 500, Debug)
│       └── layouts/      # Layout Utama
├── src/
│   ├── main.rs           # Entry point (Strict Config)
│   ├── app/              # Core Application Logic (Controllers, Middleware)
│   ├── config/           # Modular Configuration (DB, Session, Server, Log)
│   └── routes/           # Web Routes
├── storage/              # Storage (Logs, Uploads, etc.)
└── .env                  # Environment Variables (Mandatory)
```

---

## 🚀 Cara Menjalankan

### 1. Persiapan Lingkungan

Salin file `.env.example` ke `.env` dan sesuaikan pengaturan Anda:

```bash
cp .env.example .env
```

### 2. Jalankan Aplikasi

```bash
cargo serve
```

Akses aplikasi di:
👉 **[http://localhost:4000](http://localhost:4000)**

---

## 🛡️ Keamanan & Logging

- **Session Security**: Sesi disimpan di database dan divalidasi silang dengan alamat IP pengguna pada setiap request.
- **Log Management**: Gunakan `cargo rustbasic cache:clear` untuk membersihkan cache dan memotong (truncate) file log tanpa menghapusnya.
- **Key Generation**: Gunakan `cargo rustbasic key:generate` untuk membuat kunci aplikasi (`APP_KEY`) baru yang aman di file `.env`.

---

_Dibuat dengan ❤️ untuk ekosistem Rust. Arsitektur Bersih, Zero-JS, Desain Premium, Kecepatan Cahaya._
