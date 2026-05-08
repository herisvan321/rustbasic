# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic.

## 🚀 Perintah Utama (Shortcuts)
Framework ini menyediakan beberapa cara singkat untuk menjalankan perintah:

### A. Menggunakan Wrapper Script (Direkomendasikan)
Gunakan perintah `cargo rustbasic` di root proyek:
```bash
cargo rustbasic <perintah>
```

### B. Menggunakan Cargo Alias
Anda juga bisa menggunakan perintah cargo yang lebih singkat:
```bash
cargo rb <perintah>
# atau
cargo rustbasic <perintah>
```

---

## ⚡ Pengembangan (Shortcuts)

### `cargo rustbasic serve` atau `cargo rustbasic serve`
Menjalankan server dalam mode pengembangan dengan fitur:
- **Auto-Watch**: Memantau perubahan pada kode Rust, template, dan konfigurasi.
- **Live Reload**: Otomatis me-refresh browser saat Anda menyimpan perubahan.

---

## 📂 1. Generator Komponen

### `make:controller`
Membuat Controller baru di `src/app/http/controllers/`.
- Perintah: `cargo rustbasic make:controller NamaController`

### `make:model`
Membuat Entity Sea-ORM baru di `src/app/models/`.
- Perintah: `cargo rustbasic make:model Nama -m`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/`.

---

## 🔐 2. Authentication Scaffolding

### `make:auth`
Memasang sistem autentikasi lengkap dengan standar visual premium.
- Perintah: `cargo rustbasic make:auth`

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan semua migrasi database.
- Perintah: `cargo rustbasic migrate`

### `db:seed`
Menjalankan seluruh database seeder yang terdaftar di `src/app/seeder.rs`.
- Perintah: `cargo rustbasic db:seed`

### `make:seeder`
Membuat file seeder baru di `database/seeders/`.

---

## 🔍 4. Monitoring

### `route:list`
Menampilkan tabel daftar rute yang aktif di aplikasi Anda.
- Perintah: `cargo rustbasic route:list`

### `check:security`
Menjalankan audit pengaturan keamanan sistem.
