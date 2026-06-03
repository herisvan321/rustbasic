# 🚀 Panduan Deployment Server VPS

## 📝 Kata Pengantar
Selamat datang di panduan **Deployment Server VPS**. Dokumentasi ini dirancang khusus untuk memandu pengembang memindahkan aplikasi web dari komputer lokal ke server produksi VPS Linux secara aman, efisien, dan berkinerja tinggi. Anda akan mempelajari alur kompilasi biner tunggal (zero-dependency build), konfigurasi service latar belakang Systemd, hingga konfigurasi proxy server Nginx dengan sertifikat enkripsi SSL gratis.

---

## 🛠️ Script Contoh

### A. Langkah Kompilasi Produksi (Build Pipeline)
```bash
# 1. Kompilasi aset React ke folder public/build
npm run build

# 2. Kompilasi biner Rust dengan optimasi release penuh
cargo build --release
```

### B. Berkas Service Systemd (`/etc/systemd/system/rustbasic.service`)
```ini
[Unit]
Description=RustBasic Web Application
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/app
# Catatan: Sesuaikan nama biner "rustbasic" jika Anda mendefinisikan BUILD_NAME di file .env
ExecStart=/var/www/app/rustbasic
Restart=always

[Install]
WantedBy=multi-user.target
```

### C. Konfigurasi Nginx Server Block (Reverse Proxy)
```nginx
server {
    listen 80;
    server_name domainanda.com;

    location / {
        proxy_pass http://127.0.0.1:4000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 🛠️ Opsi Pilihan Build Produksi & Kompilasi Silang

Dengan menggunakan perintah `rustbasic build`, Anda akan dipandu melalui antarmuka CLI interaktif untuk menentukan target build:

1. **Pilihan Target OS**:
   - Native (OS saat ini)
   - Windows (`x86_64-pc-windows-msvc`)
   - Linux GNU (`x86_64-unknown-linux-gnu` / `aarch64-unknown-linux-gnu`)
   - Linux MUSL (`x86_64-unknown-linux-musl` / `aarch64-unknown-linux-musl`) - Sangat cocok untuk *static binary* mandiri.
   - macOS ARM & Intel

2. **Opsi Compiler**:
   - Jika sistem Anda terpasang `cargo-zigbuild`, Anda dapat memilih menggunakan **Zigbuild** untuk kompilasi silang (*cross-compilation*) tanpa perlu memasang toolchain target secara lokal.

3. **Penamaan Biner Kustom (`BUILD_NAME`)**:
   Untuk mengganti nama file biner hasil build di dalam folder `deploy/`, Anda cukup menambahkan konfigurasi berikut pada file `.env` di root proyek Anda:
   ```env
   BUILD_NAME=nama_aplikasi_anda
   ```
   Setelah build selesai, biner di folder `deploy/` akan otomatis disalin dengan nama `nama_aplikasi_anda` (atau `nama_aplikasi_anda.exe` di Windows).

---

## 🔄 Perbandingan Pemakaian (Traditional Node/PHP Deploy vs Rust Single-Binary Deploy)

Berikut adalah perbandingan pemakaian antara deployment website dinamis konvensional dan deployment biner terkompilasi RustBasic:

| Parameter Deployment | Server Tradisional (PHP / Node) | Server Biner Tunggal Rust |
| :--- | :--- | :--- |
| **File yang Diunggah** | Ratusan file kode sumber mentah + ribuan dependensi. | Hanya **satu file biner terkompilasi** + file `.env`. |
| **Kebutuhan Runtime** | Harus memasang runtime interpreter (PHP-FPM / Node.js). | Tidak membutuhkan runtime tambahan (zero-dependency). |
| **Penyajian Aset** | Membaca file gambar/CSS/JS dari disk secara konvensional. | Membaca aset super cepat langsung dari RAM memori biner. |
| **Beban Memory (RAM)** | Tinggi (karena interpreter membaca kode saat runtime). | Sangat rendah (efisien karena sudah berupa kode mesin). |

---

## 📊 Tabel Ringkasan Berkas Minimum di Server Produksi

Berikut adalah berkas penting yang wajib diletakkan di direktori `/var/www/app/` server produksi Anda:

| Nama Berkas / Folder | Kebutuhan | Deskripsi Peran di Server |
| :--- | :--- | :--- |
| **`rustbasic`** (atau nama kustom dari `BUILD_NAME`) | Wajib | Berkas biner executable utama hasil kompilasi produksi. |
| **`.env`** | Wajib | Berkas teks konfigurasi port server, kunci keamanan, & info db produksi. |
| **`storage/`** | Wajib | Folder untuk menampung rekaman log sistem (`storage/logs/`). |
| **`database/`** | Opsional | Tempat file SQLite tersimpan jika menggunakan driver SQLite. |

---

## 🏁 Penutup
Deployment berbasis file biner mandiri memberikan efisiensi luar biasa pada performa server VPS Anda, mempercepat waktu rilis aplikasi, serta memperketat keamanan karena tidak ada file kode sumber mentah (.rs/.jsx) yang disimpan di server produksi.
