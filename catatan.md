# 📘 Catatan Dokumentasi RustBasic (HTMX Edition)

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Monolith dengan filosofi HTMX & Pure CSS).

---

## 📂 1. Struktur Folder (Modular & Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar)
├── src/resources/
│   └── views/            # Template HTML (Minijinja - .rb.html)
│       └── layouts/      # Layout Utama
├── src/
│   ├── main.rs           # Entry point (Strict Config & Mandatory .env)
│   ├── app/              # Folder Inti Aplikasi (Controllers, Middleware)
│   ├── config/           # Pusat Konfigurasi (Server, Session, Logging)
│   └── routes/           # Pengaturan rute
├── storage/              # Penyimpanan File & Log
└── .env                  # Environment Variables (Wajib Ada)
```

---

## ⚙️ 2. Konfigurasi & Keamanan (Hardened)

Aplikasi menerapkan standar keamanan produksi:

- **Mandatory .env**: Aplikasi akan `panic!` jika file `.env` tidak ditemukan untuk mencegah salah konfigurasi.
- **Session-IP Binding**: Setiap sesi dikunci ke alamat IP pembuatnya. Jika IP berubah secara drastis saat sesi aktif, sistem akan menolak akses untuk mencegah pembajakan sesi.
- **Dual Logging**: 
    - Terminal: Output berwarna untuk visibilitas instan.
    - File: Log bersih (tanpa kode warna ANSI) di `storage/logs/` untuk audit.
- **Cache:Clear**: Perintah `cargo rustbasic cache:clear` sekarang memotong (truncate) file log tanpa menghapusnya, menjaga kompatibilitas dengan server yang sedang berjalan.
- **Flexible Assets & Binary Embedding**: Library HTMX dan File CSS inti dapat ditanam ke dalam file eksekusi (binary) aplikasi untuk performa maksimal. Namun, framework kini juga mendukung penggunaan **CDN eksternal** secara fleksibel, memudahkan integrasi library pihak ketiga tanpa harus meng-host file tersebut secara lokal.
- **Browser Live Reload**: Menggunakan `tower-livereload` yang hanya aktif jika `APP_DEBUG=true`. Fitur ini memungkinkan browser melakukan refresh otomatis setiap kali server melakukan restart atau ada perubahan pada file template/aset.
- **Source Minification**: Output HTML secara default diminifikasi oleh server (spasi/komentar dihapus) untuk melindungi struktur source code.
- **Hybrid Embedding (rust-embed)**: Seluruh folder template (`src/resources/views`) kini di-embed ke dalam binary saat kompilasi rilis. Saat pengembangan (debug), aplikasi tetap membaca dari disk untuk mendukung *Live Reload*.
- **Modern Premium UI**: Framework kini mewajibkan standar desain tinggi (Split-Screen, Glassmorphism) untuk semua modul inti seperti Autentikasi dan Dashboard.

---

## 🎨 3. Frontend & UI (HTMX & Pure CSS Philosophy)

RustBasic meninggalkan library JavaScript berat (seperti Alpine.js) dan beralih ke **Murni HTMX + CSS**:

- **HTML Murni & Jinja**: Menggunakan ekstensi `.rb.html`. Tidak menggunakan sistem komponen reaktif yang ajaib, murni HTML dengan kelas utilitas.
- **Floating Alerts**: Notifikasi tidak lagi mendorong konten, melainkan melayang di pojok kanan atas dengan animasi halus.
- **SPA Experience**: Navigasi instan menggunakan `hx-boost` dan `hx-indicator` untuk feedback visual.
- **Modularity via Jinja**: Jika Anda membutuhkan komponen berulang (seperti tombol khusus atau form layout), gunakan fitur bawaan Minijinja seperti `{% include %}` atau `{% macro %}`.

---

## 🗄️ 4. Database & Time Management

- **Multi-Database**: Mendukung SQLite dan MySQL via `sqlx::AnyPool`.
- **RustBasicSessionStore**: Menyimpan IP Address untuk setiap sesi guna keamanan ekstra.
- **Timezone Aware**: Semua fungsi waktu merujuk pada `APP_TIMEZONE` di `.env`. Menggunakan `chrono-tz` untuk konversi zona waktu yang akurat (WIB, UTC, dll).
- **📧 Mail Service (Lettre)**: Integrasi sistem pengiriman email asinkron (SMTP) untuk notifikasi dan pemulihan akun.
- **🔑 Password Recovery**: Sistem reset password otomatis dengan token aman (UUID) dan masa berlaku 60 menit.

---

## 🚀 5. Perintah Pengembangan (CLI)

```bash
cargo serve                        # Menjalankan server (Auto-Reload + Live Browser Refresh)
cargo watch -w src -w resources -x run # Manual Watch (Rust + Templates)
cargo rustbasic key:generate      # Membuat APP_KEY baru di file .env
cargo rustbasic cache:clear       # Truncate logs & sessions

## Menjalankan Migrasi
cargo rustbasic migrate
cargo rustbasic migrate:refresh   # Reset semua dan jalankan ulang
cargo rustbasic migrate:back      # Rollback 1 langkah

cargo rustbasic route:list         # Menampilkan daftar route dalam tabel
cargo rustbasic build              # Menu build interaktif
cargo rustbasic auth               # Scaffold autentikasi HTML (Login/Register/Reset)
cargo rustbasic auth:back          # Menghapus scaffold autentikasi secara bersih
```

---

_Dokumentasi ini diperbarui pada Mei 2026 mencerminkan transisi ke format HTML murni (.rb.html), Arsitektur HTMX, Hardened Session Security, dan CLI baru._
