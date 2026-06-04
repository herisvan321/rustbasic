# 🚀 Panduan Deployment Server VPS

## 📝 Kata Pengantar
Selamat datang di panduan **Deployment Server VPS**. Dokumentasi ini dirancang khusus untuk memandu pengembang memindahkan aplikasi web dari komputer lokal ke server produksi VPS Linux secara aman, efisien, dan berkinerja tinggi. 

Anda akan mempelajari alur kompilasi biner tunggal (zero-dependency build), konfigurasi service latar belakang Systemd, konfigurasi proxy server Nginx dengan sertifikat enkripsi SSL gratis, hingga manajemen hak akses file database.

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
Systemd digunakan untuk mengelola aplikasi RustBasic agar berjalan sebagai daemon (proses latar belakang) di Linux, otomatis berjalan saat booting, dan mendeteksi restart jika biner crash.
```ini
[Unit]
Description=RustBasic Web Application
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/app
# Mengarahkan ExecStart ke berkas biner utama Anda
ExecStart=/var/www/app/rustbasic
Restart=always
# Batasi penulisan log berlebih di syslog
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

### C. Konfigurasi Nginx Server Block (Reverse Proxy)
Nginx bertindak sebagai gerbang terdepan (reverse proxy) yang menyalurkan request publik ke port lokal aplikasi RustBasic.
```nginx
server {
    listen 80;
    server_name domainanda.com;

    location / {
        proxy_pass http://127.0.0.1:4000; # Sesuaikan port dengan PORT di file .env Anda
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 🏗️ Pilihan Kompilasi Produksi (Target Builds)

Dengan menggunakan perintah `rustbasic build`, Anda dapat memicu kompilasi produksi. Memahami target sistem operasi server Anda sangat penting:

### 1. GNU Target (`x86_64-unknown-linux-gnu`)
- **Karakteristik**: Target default untuk sebagian besar server Linux (Ubuntu, Debian, CentOS). Biner bergantung secara dinamis pada pustaka sistem C library (`glibc`).
- **Prasyarat**: Versi `glibc` di server VPS Anda harus sama atau lebih baru daripada versi `glibc` di mesin kompilasi Anda.

### 2. MUSL Target (`x86_64-unknown-linux-musl`)
- **Karakteristik**: Mengompilasi seluruh dependensi (termasuk C library) secara statik langsung ke dalam berkas biner (*fully static binary*).
- **Kelebihan**: Biner mandiri tanpa dependensi dinamis (*zero-dependency*). Biner ini dijamin berjalan di semua distro Linux apa pun tanpa memandang versi library lokal server Anda.

### 3. Kompilasi Silang (Cross-Compilation)
Apabila Anda melakukan development di macOS atau Windows tetapi ingin merilis ke Linux VPS, Anda dapat menginstal `cargo-zigbuild` untuk melakukan kompilasi silang secara instan tanpa memerlukan toolchain target yang rumit:
```bash
cargo zigbuild --target x86_64-unknown-linux-musl --release
```

---

## ⚙️ Mengelola Service Systemd di Server VPS

Setelah Anda menyalin berkas konfigurasi `/etc/systemd/system/rustbasic.service` ke server, jalankan perintah-perintah berikut untuk mengontrol siklus proses aplikasi:

```bash
# 1. Muat ulang konfigurasi Systemd dari disk
sudo systemctl daemon-reload

# 2. Aktifkan service agar otomatis berjalan saat server booting
sudo systemctl enable rustbasic

# 3. Jalankan aplikasi RustBasic sekarang
sudo systemctl start rustbasic

# 4. Memeriksa status kesehatan service
sudo systemctl status rustbasic

# 5. Memantau catatan log aplikasi secara real-time
journalctl -u rustbasic -f
```

---

## 🔒 Mengamankan Nginx dengan SSL HTTPS (Certbot)

Untuk mengamankan komunikasi data browser klien dengan server VPS menggunakan SSL/TLS HTTPS gratis dari Let's Encrypt, gunakan Certbot:

```bash
# 1. Install Certbot beserta plugin Nginx di Ubuntu/Debian
sudo apt update
sudo apt install certbot python3-certbot-nginx -y

# 2. Picu konfigurasi otomatis SSL Certbot untuk domain Anda
sudo certbot --nginx -d domainanda.com -d www.domainanda.com
```
Certbot secara otomatis memodifikasi file konfigurasi server block Nginx Anda untuk menyuntikkan SSL certificate, mengonfigurasi auto-renewal, dan merutekan traffic HTTP (port 80) ke HTTPS (port 443) dengan aman.

---

## 🛠️ Konfigurasi Hak Akses Direktori (Permissions)

> [!CAUTION]
> **Error: Database Locked / Permission Denied**
> Apabila Anda menggunakan database bertipe **SQLite** atau menyimpan file unggahan lokal, pastikan user daemon `www-data` (atau user yang Anda definisikan di Systemd) memiliki hak untuk membaca dan menulis di folder database dan storage.

Jalankan perintah-perintah berikut di server VPS Anda untuk mengatur hak kepemilikan direktori secara tepat:
```bash
# Ubah kepemilikan folder proyek secara rekursif ke user www-data
sudo chown -R www-data:www-data /var/www/app

# Berikan izin tulis penuh pada folder storage dan database
sudo chmod -R 775 /var/www/app/storage
sudo chmod -R 775 /var/www/app/database
```

---

## 🔄 Perbandingan Pemakaian (Traditional Node/PHP Deploy vs Rust Single-Binary Deploy)

Berikut adalah perbandingan pemakaian antara deployment website dinamis konvensional dan deployment biner terkompilasi RustBasic:

| Parameter Deployment | Server Tradisional (PHP / Node) | Server Biner Tunggal Rust |
| :--- | :--- | :--- |
| **File yang Diunggah** | Ratusan file kode sumber mentah + ribuan dependensi. | Hanya **satu file biner terkompilasi** + file `.env`. |
| **Kebutuhan Runtime** | Harus memasang runtime interpreter (PHP-FPM / Node.js). | Tidak membutuhkan runtime tambahan (zero-dependency). |
| **Penyajian Aset** | Membaca file gambar/CSS/JS dari disk secara konvensional. | Membaca aset super cepat langsung dari RAM memori biner. |
| **Beban Memory (RAM)** | Tinggi (karena interpreter membaca kode saat runtime). | Sangat rendah (efisien karena sudah berupa kode mesin). |
| **Keamanan Kode Sumber**| Kode sumber mentah (.js/.php) tersimpan rentan di server. | Hanya biner terkompilasi, menyembunyikan logika internal. |

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
Deployment berbasis file biner mandiri memberikan efisiensi luar biasa pada performa server VPS Anda, mempercepat waktu rilis aplikasi, serta memperketat keamanan karena tidak ada file kode sumber mentah yang disimpan di server produksi.
