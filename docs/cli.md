# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic.

## 🚀 Perintah Utama
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah:

```bash
cargo rustbasic <perintah> [argumen]
```

Atau gunakan alias `rb` jika Anda telah menambahkannya ke shell profile Anda:
```bash
rb <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

### `cargo serve`
Menjalankan server dalam mode pengembangan dengan fitur:
- **Auto-Watch**: Memantau perubahan pada kode Rust (`src/`), template (`.rb.html`), dan konfigurasi (`.env`).
- **Live Reload**: Otomatis me-refresh browser saat Anda menyimpan perubahan template.
- **Template Rendering**: Mengkompilasi dan merender file `.rb.html` secara otomatis.

---

## 📂 1. Generator Komponen

### `make:controller`
Membuat Controller baru di `src/app/http/controllers/`.
- Secara otomatis mereferensikan template `.rb.html`.
- Otomatis mendaftarkannya di `mod.rs`.

### `make:model`
Membuat Entity Sea-ORM baru di `src/app/models/`.
- Gunakan `-m` untuk sekaligus membuat file migrasi.

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/` dan mendaftarkannya.

---

## 🔐 2. Authentication Scaffolding

### `auth` / `make:auth`
Memasang sistem autentikasi lengkap dengan standar visual premium:
- **Views**: Membuat halaman Login, Register, dan Dashboard menggunakan **Modern Split-Screen UI** (Glassmorphism & CSS Mesh Gradient).
- **Floating Toasts**: Mengintegrasikan sistem notifikasi melayang yang elegan dengan auto-dismiss.
- **Logic**: Mengintegrasikan sistem Session, Bcrypt, dan Middleware secara otomatis.

### `auth:back`
Menghapus seluruh sistem autentikasi yang dibuat oleh `make:auth`.

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan semua migrasi database yang belum dieksekusi.

### `migrate:refresh`
Melakukan *rollback* pada semua migrasi yang ada, lalu menjalankan semuanya kembali dari awal. Sangat berguna untuk mereset skema database di lingkungan pengembangan.

### `migrate:back` / `migrate:rollback`
Membatalkan eksekusi migrasi yang terakhir (mundur 1 langkah).

### `cache:clear`
Membersihkan semua sesi di database dan file log lama.

### `key:generate`
Membuat token `APP_KEY` unik di file `.env`.

---

## 🔍 4. Monitoring

### `route:list`
Menampilkan tabel daftar rute yang aktif di aplikasi Anda (Method, Path, dan Handler).

### `check:security`
Menjalankan audit pengaturan keamanan sistem (CSRF, tipe DB, mode Debug).

### `check:update`
Memeriksa pembaruan versi dependensi di crates.io.

---

## 🏗️ 5. Build Manager

### `build`
Menu interaktif untuk melakukan kompilasi aplikasi ke berbagai target OS (Windows, Linux, macOS) dengan optimasi produksi.
