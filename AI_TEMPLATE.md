# 🎨 RustBasic AI Template Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent saat melakukan porting atau implementasi template.html ke dalam framework **RustBasic**.

---

## 📥 1. INPUT (Analisis & Persiapan)
Sebelum melakukan modifikasi file, AI harus mengumpulkan data berikut:
- **Nama Template**: Nama ini akan menjadi nama folder utama.
- **Source Code**: Gunakan `template.html` sebagai sumber utama. **DILARANG** mengubah atau menghapus file `template.html` ini.
- **Identifikasi Aset**: Scan file source untuk menemukan blok `<style>`, `<script>`, dan struktur HTML utama.

---

## ⚙️ 2. PROSES (Teknis Pemisahan)
Lakukan pemisahan kode secara sistematis berdasarkan folder mapping:

### A. Ekstraksi CSS
- Buat folder: `resources/css/<template_name>/`.
- Pindahkan semua CSS dari tag `<style>` ke `resources/css/<template_name>/style.css`.
- Jika ada file CSS eksternal, download/copy ke folder tersebut.

### B. Ekstraksi HTML & Layout
- Buat folder: `resources/views/<template_name>/`.
- **Layout Baru**: Buat file `resources/views/<template_name>/layout.html` untuk menyimpan struktur dasar (head, nav, sidebar, footer).
- **Index View**: Buat file `resources/views/<template_name>/index.html` yang melakukan `{% extends "<template_name>/layout.html" %}`.
- **PENTING**: Dilarang mengubah file di folder `resources/views/layouts/` yang sudah ada.

### C. Ekstraksi Komponen
- Identifikasi bagian UI yang berulang atau memiliki logika terpisah (contoh: Navbar, Sidebar, Card, Table, Footer).
- Buat file baru: `resources/views/components/<template_name>.html`.
- Pindahkan bagian-bagian tersebut ke dalam file ini menggunakan Jinja Macros (`{% macro ... %}`).
- Panggil komponen di dalam View atau Layout menggunakan `{% from "components/<template_name>.html" import ... %}`.

### D. Ekstraksi JavaScript
- Buat folder: `resources/js/<template_name>/`.
- Pindahkan script dari tag `<script>` ke `resources/js/<template_name>/script.js`.

### D. Routing & Controller
- Tambahkan endpoint baru di `src/routes/web.rs` yang mengarah ke template tersebut.
- Buat controller di `src/app/http/controllers/` untuk memanggil view baru.
- Contoh: `view(&req, "<template_name>/index.html", context! { ... })`.

---

## 📤 3. OUTPUT (Struktur File Akhir)
Hasil akhir harus mengikuti pola folder berikut:
```text
resources/
├── css/
│   └── <template_name>/
│       └── style.css
├── js/
│   └── <template_name>/
│       └── script.js
└── views/
    ├── components/
    │   └── <template_name>.html (Berisi Macros)
    └── <template_name>/
        ├── layout.html
        └── index.html
```

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan)
- **Source Protection**: Jangan pernah menghapus atau memodifikasi file `template.html`.
- **No Overwrite**: Dilarang merubah layout lama atau file yang sudah ada di folder lain. Semua perubahan harus terisolasi di dalam folder `<template_name>`.
- **No Inline**: Dilarang membiarkan inline CSS atau inline JS di dalam file HTML view.
- **No CDN**: Semua aset harus bersifat lokal (Offline First).
- **HTMX First**: Jika ada interaksi JS yang bisa digantikan dengan HTMX, AI wajib menyarankannya.

---

## 🛠️ 5. ACTION (Verifikasi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | Menjalankan server untuk melihat hasil render template. |
| `cargo rustbasic cache:clear` | Jalankan jika perubahan CSS/JS tidak langsung terlihat. |

---
_Instruksi ini melengkapi `agents.md` khusus untuk bagian manajemen template UI._
