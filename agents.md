# 🤖 RustBasic AI Agents Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent (seperti Antigravity, Cursor, dll) saat memodifikasi atau mengembangkan fitur di dalam framework **RustBasic**.

---

## 🏗️ 0. GOLDEN RULES (Prinsip Utama)
1. **HTML & Minijinja Syntax**: WAJIB menggunakan sintaks HTML standar dan Minijinja (`{{ var }}`, `{% block %}`) di dalam file `.rb.html`.
2. **HTMX & Pure CSS Philosophy**: SEMUA interaksi dinamis menggunakan **HTMX** dan UI menggunakan **Pure CSS**.
3. **Core vs App**: Logika utama framework berada di library `rustbasic-core`. AI Agent hanya boleh memodifikasi folder `src/` aplikasi kecuali diminta secara eksplisit untuk memperbaiki framework.
4. **Modern Aesthetics**: UI WAJIB terlihat premium, modern (split-screen, glassmorphism), dan responsif.

---

## 🛠️ 1. PERINTAH EKSEKUSI (CLI)
AI Agent harus menggunakan wrapper `cargo rustbasic` untuk semua tugas pengembangan:

| Tugas | Perintah |
| :--- | :--- |
| **Jalankan Server** | `cargo rustbasic serve` |
| **Buat Controller** | `cargo rustbasic make:controller <Name>` |
| **Pasang Auth** | `cargo rustbasic make:auth` |
| **Migrasi DB** | `cargo rustbasic migrate` |
| **Seeding DB** | `cargo rustbasic db:seed` |
| **Cek Route** | `cargo rustbasic route:list` |

---

## ⚙️ 2. PROCESS (Langkah Kerja Teknis)

### A. Routing & Controller
1. Tambahkan rute di `src/routes/web.rs`.
2. Buat controller di `src/app/http/controllers/`. Gunakan pola ini:
```rust
use crate::app::view;
use rustbasic_core::requests::Request;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn name(req: Request) -> impl IntoResponse {
    view(&req, "page_name.rb.html", context! {
        data => "value"
    })
}
```

### B. Database (Sea-ORM)
1. Buat model/migration: `cargo rustbasic make:model Name -m`.
2. Edit file migration di `database/migrations/`.
3. Jalankan: `cargo rustbasic migrate`.

### C. Frontend (HTML & Minijinja)
1. Gunakan `{% extends "layouts/app.rb.html" %}` di setiap halaman baru.
2. Gunakan tag HTML standar untuk elemen UI.
3. Gunakan atribut HTMX untuk interaksi: `hx-post`, `hx-target`, `hx-indicator`.
4. Panggil `{{ app_css() | safe }}` dan `{{ htmx_js() | safe }}` pada layout utama.

---

## 📂 3. FOLDER MAPPING
| Area | Path Folder |
| :--- | :--- |
| **Logika Bisnis** | `src/app/http/controllers/` |
| **Model DB** | `src/app/models/` |
| **Template** | `src/resources/views/` |
| **Migrasi** | `database/migrations/` |

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic Framework._
