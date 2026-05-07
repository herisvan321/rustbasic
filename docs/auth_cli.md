# 🔐 RustBasic Authentication CLI

Dokumentasi fitur *scaffolding* autentikasi otomatis pada framework RustBasic.

---

## 🚀 Perintah Utama

### 1. Memasang Autentikasi (`auth`)
Membangun sistem login, registrasi, forgot password, reset password, dan dashboard secara otomatis dengan desain premium.

```bash
cargo rustbasic auth
```

**Fitur Unggulan:**
- **Modern Split-Screen UI**: Desain antarmuka kelas atas dengan estetika *Glassmorphism* dan *Mesh Gradient*.
- **HTML & Minijinja Native**: Menggunakan sintaks HTML standar dan power dari engine Minijinja (`.rb.html`).
- **HTMX Powered**: Interaksi form tanpa reload menggunakan HTMX (termasuk validasi inline).
- **Floating Toasts**: Sistem pesan sukses/error melayang dengan animasi CSS otomatis (tanpa JS tambahan).
- **Secure by Default**: Terproteksi oleh `csrf_middleware` dan validasi *server-side* yang kuat.

---

## 📂 Struktur File Tergenerasi

Setelah menjalankan `auth`, file-file berikut akan dibuat:

| Path | Keterangan |
| :--- | :--- |
| `src/resources/views/auth/login.rb.html` | Halaman login dengan desain split-screen modern. |
| `src/resources/views/auth/register.rb.html` | Halaman registrasi dengan validasi real-time. |
| `src/resources/views/auth/forgot.rb.html` | Alur pemulihan password. |
| `src/resources/views/dashboard.rb.html` | Dashboard administrator premium dengan statistik grid. |
| `src/app/http/controllers/auth/` | Pusat logika autentikasi (Login, Register, Reset). |

---

## 🗑️ Menghapus Autentikasi (`auth:back`)
Jika Anda ingin membersihkan alur auth dan memulai dari awal:

```bash
cargo rustbasic auth back
```
Atau:
```bash
cargo rustbasic auth:back
```

---

## 🛠️ Kustomisasi
Anda dapat memodifikasi logika di `src/app/http/controllers/auth/auth_controller.rs` dan memperbarui tampilan langsung di file `.rb.html` terkait. Setiap perubahan pada template akan memicu auto-refresh di browser jika `cargo serve` sedang berjalan.
