# 🎨 RustBasic AI Template Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent saat melakukan porting atau implementasi template ke dalam framework **RustBasic RSX**.

---

## 📥 1. INPUT (Analisis & Persiapan)
Sebelum melakukan modifikasi file, AI harus mengumpulkan data berikut:
- **Nama Template**: Nama ini akan menjadi nama folder utama.
- **Source Code**: Gunakan file sumber (misal `template.html`) sebagai sumber utama.
- **Identifikasi Aset**: Scan file source untuk menemukan blok `<style>`, `<script>`, dan struktur HTML utama.

---

## ⚙️ 2. PROSES (Teknis Pemisahan & RSX)

### A. Ekstraksi CSS
- Buat folder: `src/resources/css/<template_name>/`.
- Pindahkan semua CSS dari tag `<style>` ke `src/resources/css/<template_name>/style.css`.

### B. Ekstraksi HTML & Layout (RSX Syntax)
- Buat file layout baru: `src/resources/views/layouts/<template_name>.rsx`.
- Gunakan tag `<Assets.Styles />` dan `<Assets.Htmx />` untuk menyertakan aset inti.
- Buat file view di `src/resources/views/<template_name>/index.rsx`.
- Gunakan `{% extends "layouts/<template_name>.rsx" %}`.

### C. Ekstraksi Komponen (Modular RSX)
- Identifikasi bagian UI yang berulang (Navbar, Sidebar, Card, Table, Footer).
- Buat file baru: `src/resources/views/components/<template_name>.rsx` menggunakan `{% macro ... %}`.
- **PENTING**: Panggil komponen tersebut di View menggunakan sintaks `<Namespace.Component />`.
- Jangan gunakan import manual Jinja (`{% from ... %}`).

### D. Ekstraksi JavaScript & HTMX
- Buat folder: `src/resources/js/<template_name>/`.
- Pindahkan script dari tag `<script>` ke `src/resources/js/<template_name>/script.js`.
- **HTMX First**: Jika ada interaksi JS yang bisa digantikan dengan HTMX, AI wajib menyarankannya.

### E. Routing & Controller
- Tambahkan endpoint baru di `src/routes/web.rs`.
- Buat controller yang merender file `.rsx` baru.

---

## 📤 3. OUTPUT (Struktur File Akhir)
```text
src/resources/
├── css/
│   └── <template_name>/style.css
├── js/
│   └── <template_name>/script.js
└── views/
    ├── components/
    │   └── <template_name>.rsx (Namespace)
    ├── <template_name>/
    │   └── index.rsx
    └── layouts/
        └── <template_name>.rsx
```

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan)
- **RSX Native**: Semua file template WAJIB menggunakan ekstensi `.rsx`.
- **No Manual Import**: Jangan gunakan `{% from ... import ... %}`. Manfaatkan auto-import.
- **Minification Aware**: Sadari bahwa output akhir akan diminifikasi oleh server untuk perlindungan source code.
- **No Inline**: Dilarang membiarkan inline CSS atau inline JS di dalam file HTML view.

---

## 🛠️ 5. ACTION (Verifikasi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | Menjalankan server untuk melihat hasil render template (Auto-transpile). |
| `cargo rustbasic cache:clear` | Jalankan jika perubahan CSS/JS tidak langsung terlihat. |

---

_Instruksi ini melengkapi `agents.md` khusus untuk bagian manajemen template UI._
/Users/herisvanhendra/Desktop/Desktop new/project/belajar rust/rustmyadmin/AI_TEMPLATE.md