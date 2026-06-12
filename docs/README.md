# 📚 Indeks Dokumentasi RustBasic Framework — Modern SPA Edition

## 📝 Kata Pengantar
Selamat datang di indeks resmi **Dokumentasi RustBasic Framework — Modern SPA Edition**. Halaman indeks ini dirancang khusus untuk memandu pengembang menemukan materi pembelajaran teknis secara cepat, komprehensif, dan sistematis. 

Mulai dari pemula yang baru mengenal konsep web SPA hingga arsitek sistem yang ingin melakukan kustomisasi tingkat lanjut, seluruh panduan disusun secara terstruktur di bawah ini.

---

## 📂 Peta Navigasi Dokumentasi

### 🚀 1. Dasar-Dasar Pengembangan (Basics)
*   **[Dasar-Dasar SPA](basics.md)**: Konsep SPA monolith, inisiasi routing web, controller, dan manajemen data.
*   **[Views & TSX Komponen](views.md)**: Panduan membangun antarmuka premium dengan React, Tailwind CSS v3, Inertia form helper, dan asset embedding.
*   **[Perutean Inline Cepat](inline_routing.md)**: Cara cepat mendefinisikan rute dan langsung mengembalikan respon secara inline menggunakan closure.
*   **[Rute Bernama (Named Routes)](named_routes.md)**: Panduan lengkap mendaftarkan rute dengan nama unik di backend Rust dan menyelesaikannya secara dinamis di React SPA.
*   **[Manajemen Database & Migrasi](database.md)**: Pemilihan driver database (SQLite/MySQL via Cargo features), deklarasi model (`model!`), eksekusi kueri CRUD, skema migrasi tabel, serta pengisian data awal seeder.
*   **[Pengolahan String & Fluent Stringable](strings.md)**: Manipulasi string static modular, pencarian karakter, casing generator, dan fluent string builder.
*   **[Unit & Feature Testing](testing.md)**: Panduan lengkap pengujian unit terisolasi serta simulasi request HTTP secara in-memory.

### ⚙️ 2. Arsitektur, Protokol, & Keamanan (Core System)
*   **[Routing & HTTP Stack](http.md)**: Detail mengenai middleware, penanganan request/response Inertia, dan keamanan proteksi CSRF otomatis.
*   **[Arsitektur Framework](architecture.md)**: Membedah struktur delegasi CLI, core library, dan pengiriman data *Shared Props* global.
*   **[Advanced Topics](advanced.md)**: Validasi input struct tingkat lanjut, sesi terenkripsi, daily rolling logging (tracing), penanganan error visual, dan cache-busting aset.

### 🛠️ 3. Perkakas CLI & Scaffolding (Command Line)
*   **[CLI Reference](cli.md)**: Daftar lengkap perintah utilitas `rustbasic` di terminal untuk inisiasi proyek & generator file otomatis.
*   **[Auth Scaffolding (Breeze)](auth_cli.md)**: Dokumentasi setup sistem autentikasi otomatis yang premium.

### 🚢 4. Produksi & Deployment (Production Ready)
*   **[Panduan Deployment VPS](deployment.md)**: Alur pembangunan kompilasi produksi (*build pipeline*) dan rilis satu file biner mandiri (*zero-dependency deployment*).

### 📦 5. Paket Tambahan (Optional Packages)
*   **[JWT Authentication (rustbasic-jwt)](jwt.md)**: Sistem autentikasi token JWT state-less terintegrasi blacklist DB dan middleware.
*   **[WebP Image Optimizer (rustbasic-webp)](webp.md)**: Utilitas optimasi gambar WebP, metadata extraction, dan aspect-ratio preserved resizing.
*   **[Media Library Management (rustbasic-medialibrary)](medialibrary.md)**: Upload manager dengan opsi penyimpanan Local/S3, auto-category, transcode WebP, dan thumbnail generator.
*   **[Role & Permission RBAC (rustbasic-permission)](permission.md)**: Manajemen wewenang pengguna dinamis dinas tabel relasi otomatis.
*   **[Multi-Language Translation (rustbasic-translatable)](translatable.md)**: Lokalisasi teks terjemahan i18n JSON dengan auto-detect language middleware.

---

## 🛠️ Script Contoh (Quick Start)

Berikut adalah contoh skrip inisialisasi cepat proyek baru menggunakan CLI global:

```bash
# 1. Menginstal CLI pembantu secara global
cargo install rustbasic-cli

# 2. Membuat kerangka proyek SPA baru
rustbasic new aplikasi-spa-saya

# 3. Menjalankan server pengembangan (Hot Reload)
cd aplikasi-spa-saya
rustbasic serve
```

---

## 🔄 Perbandingan Pemakaian Alur Belajar

Berikut adalah perbandingan target pemakai berdasarkan materi dokumentasi:

| Dokumen Panduan | Cocok Untuk Pemula (Beginner) | Cocok Untuk Profesional (Advanced) |
| :--- | :--- | :--- |
| **Basics & Views** | Mempelajari routing & menulis komponen React. | Optimasi asset bundling dan HMR. |
| **Database & HTTP** | Membuat tabel baru & memahami CRUD dasar. | Menulis custom middleware & kueri relasi kompleks. |
| **CLI & Auth** | Menjalankan perintah generator berkas otomatis. | Konfigurasi SMTP email recovery & UUID token flow. |
| **Deployment** | Menyalin berkas biner ke server VPS. | Konfigurasi Nginx SSL & monitoring daemon Systemd. |

---

## 📊 Tabel Ringkasan Lokasi Dokumentasi Proyek

Berikut adalah daftar lokasi berkas panduan penting di dalam direktori proyek Anda:

| Berkas Panduan | Lokasi File Dokumentasi | Topik Utama yang Dibahas |
| :--- | :--- | :--- |
| **basics.md** | `docs/basics.md` | Routing, Controller, dan Views. |
| **views.md** | `docs/views.md` | React TSX, Tailwind CSS v3, & useForm. |
| **named_routes.md** | `docs/named_routes.md` | Pendaftaran rute bernama di Rust dan penyelesaian URL dinamis di React. |
| **database.md** | `docs/database.md` | Model, CRUD, Migrasi, & Seeder. |
| **strings.md** | `docs/strings.md` | Helper manipulasi string static & fluent Stringable chaining. |
| **http.md** | `docs/http.md` | Keamanan CSRF, Middleware kustom, & HTTP 303. |
| **cli.md** | `docs/cli.md` | Pintasan terminal generator perintah. |
| **testing.md** | `docs/testing.md` | Unit & Feature Testing (In-Memory). |
| **auth_cli.md** | `docs/auth_cli.md` | Integrasi Breeze Authentication Scaffolding. |
| **deployment.md** | `docs/deployment.md` | Deployment Systemd, VPS, & Nginx SSL. |
| **jwt.md** | `docs/jwt.md` | Sistem autentikasi token JWT state-less & blacklist. |
| **webp.md** | `docs/webp.md` | Konversi gambar WebP & aspect-ratio preserved resizing. |
| **medialibrary.md** | `docs/medialibrary.md` | Upload manager Local/S3, thumbnails & WebP. |
| **permission.md** | `docs/permission.md` | Otorisasi rute dinamis berbasis RBAC. |
| **translatable.md** | `docs/translatable.md` | Penerjemah teks i18n JSON & middleware locale. |


---

## 🏁 Penutup
Dengan mengikuti rute navigasi dokumentasi ini secara bertahap, Anda dapat menguasai seluruh siklus pengembangan aplikasi web modern SPA berbasis RustBasic dengan mudah dan terarah. Selamat belajar!
