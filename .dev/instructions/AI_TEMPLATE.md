# 🎨 RustBasic AI Template Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent saat melakukan porting atau implementasi template UI ke dalam framework **RustBasic**.

---

## 📥 1. INPUT (Analisis & Persiapan)
Sebelum melakukan modifikasi file, AI harus mengumpulkan data berikut:
- **Nama Template**: Nama ini akan menjadi nama folder utama.
- **Source Code**: Gunakan file sumber (misal `template.html`) sebagai referensi utama.
- **Identifikasi Aset**: Scan file source untuk menemukan blok `<style>`, `<script>`, dan struktur HTML utama.
- **Premium Aesthetics**: Wajib mengadaptasi desain menjadi modern, premium, dan responsif sesuai standar RustBasic.

---

## ⚙️ 2. PROSES (Teknis Pemisahan HTML & Aset)

### A. Ekstraksi CSS
- Buat folder: `src/resources/css/<template_name>/`.
- Pindahkan semua CSS dari tag `<style>` ke `src/resources/css/<template_name>/style.css`.

### B. Ekstraksi HTML & Layout (Standar HTML + Jinja)
- Buat file layout baru: `src/resources/views/layouts/<template_name>.rb.html`.
- Sertakan aset inti di dalam blok `<head>` dan `<body>` dengan menggunakan helper `{{ app_css() | safe }}` dan `{{ htmx_js() | safe }}` jika memungkinkan.
- Buat file view halaman spesifik di `src/resources/views/<template_name>/index.rb.html`.
- Gunakan `{% extends "layouts/<template_name>.rb.html" %}` di setiap halaman.

### C. Ekstraksi Komponen (Jinja Include/Macro Murni)
- **Tidak ada komponen ajaib RSX**. Jika bagian UI berulang (seperti Navbar, Footer), pisahkan ke dalam file `.rb.html` kecil (misal: `src/resources/views/partials/navbar.rb.html`).
- Panggil bagian tersebut menggunakan sintaks Minijinja standar: `{% include "partials/navbar.rb.html" %}`.
- Jika butuh komponen dengan parameter, gunakan sintaks `{% macro %}` bawaan Minijinja standar.

### D. Ekstraksi JavaScript & HTMX
- Buat folder: `src/resources/js/<template_name>/`.
- Pindahkan script dari tag `<script>` ke `src/resources/js/<template_name>/script.js`.
- **HTMX First**: Gantikan interaksi JS sederhana (misal modal, tab, load data) dengan atribut HTMX sebisa mungkin.

### E. Routing & Controller
- Tambahkan endpoint baru di `src/routes/web.rs`.
- Buat controller yang merender file `.rb.html` baru.
- Semua halaman pada file ini `src/resources/views/layouts/<template_name>.rb.html` harus dipisah dan dibuatkan endpoint baru di `src/routes/web.rs`.

---

## 📤 3. OUTPUT (Struktur File Akhir)
```text
src/resources/
├── css/
│   └── <template_name>/style.css
├── js/
│   └── <template_name>/script.js
└── views/
    ├── <template_name>/
    │   └── index.rb.html
    └── layouts/
        └── <template_name>.rb.html
```

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan)
- **Ekstensi RB.HTML**: Semua file template WAJIB menggunakan ekstensi `.rb.html`.
- **Standar HTML**: Gunakan HTML murni. Sistem komponen bergaya JSX (seperti `<Buttons.Button />`) **sudah dihapus**.
- **Hybrid Embedding**: Ingat bahwa template di-embed via `rust-embed` namun mendukung live-reload di mode debug.
- **Minification Aware**: Sadari bahwa output akhir akan diminifikasi oleh server (spasi/komentar dihapus) untuk perlindungan source code.
- **No Inline**: Usahakan tidak membiarkan inline CSS atau inline JS di dalam file HTML view. Pisahkan ke file tersendiri.

---

## 🛠️ 5. ACTION (Verifikasi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo rustbasic serve` | Menjalankan server untuk melihat hasil render template dengan Auto-reload. |
| `cargo rustbasic cache:clear` | Jalankan jika perubahan cache/log perlu dibersihkan. |

---

_Instruksi ini melengkapi `agents.md` khusus untuk bagian manajemen template UI._