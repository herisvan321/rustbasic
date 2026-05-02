# 📘 Catatan Dokumentasi RustBasic (Zero-JS Edition)

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Monolith dengan filosofi Zero-JS).

---

## 📂 1. Struktur Folder (Modular & Zero-JS)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar) - ZERO JS (No JS Libs)
├── resources/
│   └── views/            # Template HTML (Minijinja)
│       ├── components/   # Modular UI Library (Split by logic)
│       └── ...
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

---

## 🎨 3. Frontend & UI (Zero-JS Philosophy)

RustBasic meninggalkan library JavaScript berat (seperti Alpine.js) dan beralih ke **Murni HTMX + CSS**:

- **Modular UI Library**: Komponen dipisah menjadi file kecil:
    - `forms.html`: Penanganan input dan validasi error.
    - `buttons.html`: Tombol aksi dan navigasi.
    - `display.html`: Alert (Floating Toast), Stat Cards, Card.
    - `overlays.html`: Modal Konfirmasi menggunakan teknik **Checkbox Hack** (Tanpa JS).
    - `feedback.html`: Loading indicators dan Skeleton loading.
- **Floating Alerts**: Notifikasi tidak lagi mendorong konten, melainkan melayang di pojok kanan atas dengan animasi halus.
- **SPA Experience**: Navigasi instan menggunakan `hx-boost` dan `hx-indicator` untuk feedback visual.

---

## 🗄️ 4. Database & Session Store

- **Multi-Database**: Mendukung SQLite dan MySQL via `sqlx::AnyPool`.
- **RustBasicSessionStore**: Menyimpan IP Address untuk setiap sesi guna keamanan ekstra.

---

## 🚀 5. Perintah Pengembangan (CLI)

```bash
cargo serve                        # Menjalankan server (Auto-Reload + Watch .env)
cargo rustbasic cache:clear       # Truncate logs & clear sessions
cargo rustbasic route:list         # Menampilkan daftar route dalam tabel
cargo rustbasic build              # Menu build interaktif
```

---

_Dokumentasi ini diperbarui Mei 2026 mencerminkan Arsitektur Zero-JS, Modular Components, Hardened Session Security, dan Dual Logging._
