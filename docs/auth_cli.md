# 🔐 RustBasic Authentication CLI

Dokumentasi ini menjelaskan cara menggunakan fitur *scaffolding* autentikasi otomatis pada framework RustBasic menggunakan perintah `cargo rustbasic`.

---

## 🚀 Perintah Utama

### 1. Memasang Autentikasi (`auth`)
Gunakan perintah ini untuk membangun seluruh sistem login dan registrasi secara otomatis.

```bash
cargo rustbasic auth
```

**Apa yang dilakukan perintah ini?**
- **Routes**: Membuat `src/routes/auth.rs` dan mendaftarkannya di `web.rs`.
- **Controllers**: Membuat `AuthController.rs` (Login/Register logic) dan `DashboardController.rs`.
- **Views**: Membuat folder `resources/views/auth/` berisi template Login & Register premium, serta `dashboard.html`.
- **Integration**: Menambahkan middleware `guest` untuk halaman login dan `auth` untuk dashboard.
- **UI**: Menambahkan tombol Login/Register secara dinamis di halaman Welcome jika fitur terpasang.

### 2. Menghapus Autentikasi (`auth back`)
Gunakan perintah ini jika Anda ingin menghapus seluruh sistem autentikasi dan mengembalikan project ke kondisi bersih.

```bash
cargo rustbasic auth back
# ATAU
cargo rustbasic auth:back
```

**Fitur Unggulan Penghapusan:**
- **Robust Clean-up**: Secara otomatis membersihkan import dan deklarasi route di `web.rs`.
- **Safety**: Memastikan project tetap bisa dikompilasi setelah penghapusan dengan merapikan file `mod.rs`.

---

## 📂 Struktur File Tergenerasi

Setelah menjalankan `auth`, file berikut akan tersedia:

| Path | Keterangan |
| :--- | :--- |
| `src/routes/auth.rs` | Definisi route `/login` dan `/register`. |
| `src/app/http/controllers/auth/` | Logika backend untuk autentikasi. |
| `src/app/http/controllers/dashboard_controller.rs` | Controller untuk halaman admin/dashboard. |
| `resources/views/auth/` | Template HTML untuk halaman Login & Daftar. |
| `resources/views/dashboard.html` | Template Dashboard dengan desain premium. |

---

## 🛠️ Kustomisasi Logic

### Database & Model
Sistem ini secara default menggunakan model `users` yang ada di `src/app/models/users.rs`. Pastikan tabel `users` sudah tersedia di database Anda. Jika belum, jalankan migrasi:

```bash
cargo rustbasic migrate
```

### Validasi & Hashing
- **Hashing**: Menggunakan `bcrypt` dengan *default cost*.
- **Validation**: Menggunakan struct `RegisterRequest` dan `LoginRequest` di dalam controller yang bisa Anda ubah aturannya sesuai kebutuhan.

---

## 💡 Tips & Troubleshooting

- **404 Error**: Jika halaman `/login` tidak ditemukan setelah instalasi, pastikan `cargo serve` telah melakukan *hot-reload* dengan benar.
- **Project Broken**: Jika terjadi error kompilasi setelah menghapus auth, jalankan `cargo rustbasic auth back` sekali lagi untuk memastikan pembersihan menyeluruh.
- **Route List**: Gunakan `cargo rustbasic route:list` untuk memverifikasi endpoint yang aktif.

---
*Dokumentasi ini dibuat otomatis oleh Antigravity AI Agent.*
