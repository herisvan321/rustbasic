# 🚀 Panduan Deployment RustBasic

Dokumen ini menjelaskan cara men-deploy aplikasi RustBasic ke server produksi.

---

## 🏗️ 1. Persiapan Lingkungan Produksi

### A. Konfigurasi `.env`
Pastikan file `.env` di server produksi memiliki pengaturan keamanan yang ketat:
```ini
APP_ENV=production
APP_DEBUG=false
APP_PORT=4000
APP_HOST=127.0.0.1  # Gunakan localhost jika di balik Reverse Proxy
APP_KEY=base64:... # Wajib unik dan rahasia!
DB_CONNECTION=sqlite # Atau mysql
```

### B. Database
- **SQLite**: Pastikan folder database (atau file `.db`) memiliki izin tulis (write permission) bagi pengguna yang menjalankan aplikasi.
- **MySQL**: Pastikan database sudah dibuat dan user memiliki akses penuh.

---

## 📦 2. Kompilasi (Build)

Jalankan perintah build release untuk mengoptimalkan performa binary:

```bash
cargo build --release
```

Binary hasil build akan berada di: `target/release/rustbasic` (atau nama binary yang Anda tentukan).

### Cross-Compilation (Opsi)
Jika Anda men-deploy dari macOS ke server Linux, gunakan `cargo-zigbuild` atau `cargo rustbasic build` untuk memilih target OS yang sesuai.

---

## 🚢 3. Struktur Folder di Server

Berkat fitur **Hybrid Embedding**, aplikasi RustBasic kini benar-benar portabel. Seluruh template HTML (Views), CSS, dan JS utama sudah tertanam di dalam file binary saat Anda melakukan build release.

Berikut adalah struktur folder minimum yang diperlukan di server:

```text
/var/www/app/
├── rustbasic            # Binary aplikasi (hasil build release)
├── .env                 # Konfigurasi produksi (Wajib)
├── database/            # Database SQLite & Migrasi (Jika menggunakan SQLite)
├── public/              # File statis tambahan (Gambar luar, Favicon, dll)
└── storage/             # Log aplikasi & Upload file
```

> [!NOTE]
> **Tidak ada folder `src/`**: Anda tidak perlu lagi menyertakan folder `src/resources/views` di server produksi. Binary Anda sudah mandiri (*self-contained*).

### 📦 Izin Tulis (Permissions)
Pastikan user yang menjalankan aplikasi (misal: `www-data`) memiliki akses tulis ke folder berikut:
- **`database/`**: Agar bisa membuat/mengedit file SQLite.
- **`storage/`**: Agar bisa menulis file log dan upload.

```bash
sudo chown -R www-data:www-data database storage
sudo chmod -R 775 database storage
```

---

## 🛠️ 4. Menjalankan Aplikasi (Systemd)

Sangat disarankan menggunakan Systemd (di Linux) untuk memastikan aplikasi tetap berjalan (auto-restart) dan berjalan di background.

Buat file `/etc/systemd/system/rustbasic.service`:
```ini
[Unit]
Description=RustBasic Web Application
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/app
ExecStart=/var/www/app/rustbasic
Restart=always

[Install]
WantedBy=multi-user.target
```

Aktifkan service:
```bash
sudo systemctl enable rustbasic
sudo systemctl start rustbasic
```

---

## 🌐 5. Reverse Proxy (Nginx)

Gunakan Nginx untuk menangani SSL (HTTPS) dan meneruskan request ke aplikasi Rust.

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

## 🛡️ 6. Checklist Keamanan Produksi
- [ ] `APP_DEBUG` diset ke `false`.
- [ ] `APP_KEY` sudah diganti dengan yang baru (`cargo rustbasic key:generate`).
- [ ] Port aplikasi (4000) ditutup oleh firewall (hanya bisa diakses via Nginx).
- [ ] Log di `storage/logs/` dipantau secara berkala.
- [ ] File database SQLite tidak diletakkan di folder publik.

---

_Panduan ini dibuat untuk memastikan aplikasi RustBasic berjalan dengan aman, stabil, dan performa maksimal di lingkungan produksi._
