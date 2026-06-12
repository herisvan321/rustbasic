# 🛠️ Development Instructions (React SPA Edition)

## 📝 Kata Pengantar

Selamat datang di panduan **Development Instructions (React SPA Edition)**. Dokumentasi ini disusun secara teratur untuk membantu pengembang menjalankan server lokal, menelusuri struktur direktori utama, melakukan kompilasi aset frontend React, serta merilis biner produksi akhir framework **RustBasic**. Melalui panduan praktis ini, Anda dapat mengoptimalkan efisiensi alur kerja pengkodean Anda mulai dari tahap pengembangan hingga deployment server.

---

## 🛠️ Script Contoh

### A. Perintah Menjalankan Mode Pengembangan Lokal (Local Development)
Jalankan kedua terminal secara berdampingan untuk mengaktifkan reaktivitas HMR penuh:
```bash
# Terminal 1: Jalankan Backend RustBasic (Port 4000)
rustbasic serve

# Terminal 2: Jalankan Vite Dev Server (Port 5173)
npm run dev
```

### B. Langkah Kompilasi & Rilis Produksi (Build Pipeline)
Satukan seluruh aset frontend dan template ke dalam satu file biner mandiri:
```bash
# 1. Compile aset React.js + Inertia ke folder src/dist/
npm run build

# 2. Compile biner Rust dengan kompresi release penuh
cargo build --release
```

### C. Konfigurasi Environment File Produksi (`.env`)
```ini
# Ubah status debug menjadi false di server produksi
APP_DEBUG=false
APP_URL=https://domainanda.com
DATABASE_URL=sqlite://database/database.sqlite
```

---

## 🔄 Perbandingan Pemakaian (Debug Mode vs Production Mode)

Berikut adalah perbandingan pemakaian dan perilaku sistem saat dijalankan dalam mode pengembangan dan mode rilis:

| Aspek Operasional | Mode Pengembangan (Debug - `APP_DEBUG=true`) | Mode Rilis (Production - `APP_DEBUG=false`) |
| :--- | :--- | :--- |
| **Sumber Aset Web** | Dibaca dinamis langsung dari direktori disk lokal. | Dibaca super cepat dari RAM memori biner ter-embed. |
| **Hot Reload (HMR)** | Aktif untuk React (Vite dev) & Rust (cargo watch). | Non-aktif. File visual tertanam permanen di dalam biner. |
| **Keamanan Sistem** | Stack trace kesalahan visual ditampilkan rinci. | Menyembunyikan seluruh informasi detail error internal. |
| **Folder Source Disk** | Folder `src/` & `public/` wajib dipertahaman. | Folder `src/` & `public/` bebas dihapus dari server. |

---

## 📊 Tabel Ringkasan Lokasi Berkas Struktur Kode Utama

Berikut adalah tabel rujukan berkas penting yang sering dimodifikasi dalam proses pengembangan fitur:

| Nama Berkas / Direktori | Jalur Folder Kode | Peran & Tanggung Jawab Utama Berkas |
| :--- | :--- | :--- |
| **`main.rs`** | `src/main.rs` | Titik masuk utama backend. Menjalankan server RustBasic & memuat config. |
| **`web.rs`** | `src/routes/web.rs` | Tempat mendaftarkan rute-rute URL web yang menyajikan Inertia. |
| **`Controllers`** | `src/app/http/controllers/`| Logika bisnis kontroller RustBasic penyuplai data JSON props ke React. |
| **`Pages`** | `src/resources/js/Pages/` | Kumpulan file komponen halaman React SPA (.tsx). |
| **`app.rb.html`** | `src/resources/views/app.rb.html`| HTML root container tempat aplikasi React SPA dihidrasi (*hydrated*). |

---

## 🏁 Penutup

Alur kompilasi biner tunggal (single-binary deployment) pada RustBasic memberikan keunggulan performa kecepatan memori RAM yang luar biasa serta menyederhanakan siklus pengiriman aplikasi Anda ke server VPS produksi. Selamat berkarya dan membangun aplikasi SPA premium Anda!
