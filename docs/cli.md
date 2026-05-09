# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic.

## 🚀 Perintah Utama (Shortcuts)
Framework ini menyediakan beberapa cara singkat untuk menjalankan perintah:

### `rustbasic new`
Membuat project RustBasic baru dari template resmi.
```bash
rustbasic new myapp
```

### `rustbasic <perintah>`
Gunakan perintah `rustbasic` di root proyek untuk tugas sehari-hari:
```bash
rustbasic make:controller BlogController
```

---

## ⚡ Pengembangan (Shortcuts)

### `rustbasic serve` atau `rustbasic serve`
Menjalankan server dalam mode pengembangan dengan fitur:
- **Auto-Watch**: Memantau perubahan pada kode Rust, template, dan konfigurasi.
- **Live Reload**: Otomatis me-refresh browser saat Anda menyimpan perubahan.

---

## 📂 1. Generator Komponen

### `make:controller`
Membuat Controller baru di `src/app/http/controllers/`.
- Perintah: `rustbasic make:controller NamaController`

### `make:model`
Membuat Entity Sea-ORM baru di `src/app/models/`.
- Perintah: `rustbasic make:model Nama -m`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/`.

---

## 🔐 2. Authentication Scaffolding

### `make:auth`
Memasang sistem autentikasi lengkap dengan standar visual premium.
- Perintah: `rustbasic make:auth`

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan semua migrasi database yang belum dieksekusi.
- Perintah: `rustbasic migrate`

### `migrate:refresh`
Melakukan rollback pada seluruh migrasi dan menjalankannya kembali dari awal. Berguna untuk mereset struktur database.
- Perintah: `rustbasic migrate:refresh`

### `migrate:back` (atau `migrate:rollback`)
Membatalkan (rollback) satu langkah migrasi terakhir.
- Perintah: `rustbasic migrate:back`

### `db:seed`
Menjalankan seluruh database seeder yang terdaftar di `src/app/seeder.rs`.
- Perintah: `rustbasic db:seed`

### `make:seeder`
Membuat file seeder baru di `database/seeders/`.

---

## 🔍 4. Monitoring

### `route:list`
Menampilkan tabel daftar rute yang aktif di aplikasi Anda.
- Perintah: `rustbasic route:list`

### `check:security`
Menjalankan audit pengaturan keamanan sistem.
