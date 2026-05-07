# 🤖 RustBasic AI Agents Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent (seperti Antigravity, Cursor, dll) saat memodifikasi atau mengembangkan fitur di dalam framework **RustBasic**.

---

## 🏗️ 0. GOLDEN RULES (Prinsip Utama)
1. **HTML & Minijinja Syntax**: WAJIB menggunakan sintaks HTML standar dan Minijinja (`{{ var }}`, `{% block %}`) di dalam file `.rb.html`. Sistem komponen RSX telah dihapus sepenuhnya.
2. **HTMX & Pure CSS Philosophy**: DILARANG menambahkan library JS baru untuk interaktivitas sederhana. Semua interaksi dinamis menggunakan **HTMX** dan UI menggunakan **Pure CSS**.
3. **No Component Magic**: Jangan mencoba menggunakan `<Namespace.Component />`. Gunakan HTML standar (misal `<button class="btn">`) atau buat macro Jinja standar jika perlu penggunaan ulang.
4. **Source Protection**: Output HTML otomatis diminifikasi oleh server (spasi dihapus, komentar dibuang) untuk menyembunyikan struktur kode asli dari "View Source".
6. **Modern Aesthetics**: UI WAJIB terlihat premium, modern (split-screen, glassmorphism), dan responsif. Gunakan variabel CSS di `style.css` untuk konsistensi warna dan efek.
7. **Hybrid Embedding**: Framework menggunakan `rust-embed` untuk template. Saat pengembangan (Debug), template dibaca dari disk untuk *Live Reload*. Saat produksi (Release), template dibaca dari memory.

---

## 📥 1. INPUT & CONTEXT (Analisis Awal)
Sebelum menulis baris kode pertama, AI harus mengecek:
- **`src/routes/web.rs`**: Lihat daftar endpoint dan middleware yang aktif.
- **`src/config/requests.rs`**: Pahami fungsi helper di `Request` (misal: `req.input()`, `req.session()`).
- **`src/config/view.rs`**: Pahami logika loader Minijinja (saat ini menggunakan standar HTML murni, tidak ada transpiler regex).

---

## ⚙️ 2. PROCESS (Langkah Kerja Teknis)

### A. Routing & Controller
1. Tambahkan rute di `src/routes/web.rs`.
2. Buat controller di `src/app/http/controllers/`. Gunakan pola ini:
```rust
pub async fn name(req: Request) -> impl IntoResponse {
    view(&req, "page_name.rb.html", context! {
        data => "value"
    })
}
```

### B. Database (Sea-ORM)
1. Buat model: `cargo rustbasic make:model Name -m`.
2. Edit file migration di `database/migrations/`.
3. Jalankan: `cargo rustbasic migrate` atau `cargo rustbasic migrate:refresh` untuk reset.

### C. Frontend (HTML & Minijinja)
1. Gunakan `{% extends "layouts/app.rb.html" %}` di setiap halaman baru.
2. Gunakan tag HTML standar untuk elemen UI (form, input, button) dengan class CSS yang sesuai dari `style.css`.
3. Gunakan atribut HTMX untuk interaksi: `hx-post`, `hx-target`, `hx-indicator`.
4. Pastikan memanggil `{{ app_css() | safe }}` dan `{{ htmx_js() | safe }}` pada template minimal jika membuat layout baru di luar `app.rb.html`.
5. **Auth UI**: Saat memodifikasi halaman auth, pertahankan layout *Split-Screen* dan estetika modern yang sudah ada di generator CLI (`src/config/cli/auth.rs`).

---

## 📂 3. FOLDER MAPPING (Lokasi File)
| Area | Path Folder | Keterangan |
| :--- | :--- | :--- |
| **Logika Bisnis** | `src/app/http/controllers/` | Pusat logika request-response. |
| **Model DB** | `src/app/models/` | Definisi tabel & relasi (Entity). |
| **Middleware** | `src/app/http/middleware/` | Filter keamanan & session. |
| **Template** | `src/resources/views/` | File `.rb.html` (HTML + Minijinja). |
| **Konfigurasi** | `src/config/` | Inti engine (View, DB, Server, CLI). |

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan Ketat)
AI Agent **DILARANG** melakukan:
- **No RSX Tags**: Dilarang menggunakan tag komponen bergaya React/RSX (`<Components />`).
- **No Inline Styles**: Masukkan CSS baru ke `src/resources/css/style.css` (bukan ad-hoc di tag HTML).
- **Session Protection**: Jangan pernah menonaktifkan `csrf_middleware` atau `guest_middleware` pada rute sensitif.
- **Logging**: Jangan menghapus `tracing::debug!` atau `tracing::info!` yang sudah ada.

---

## 🛠️ 5. ACTION (Perintah Eksekusi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | **Wajib dipakai** (Auto-Reload + Live Browser Refresh). |
| `cargo rustbasic make:controller <Name>` | Menghasilkan boilerplate controller yang merujuk ke `.rb.html`. |
| `cargo rustbasic auth` | Memasang sistem auth lengkap dengan sintaks HTML murni. |
| `cargo rustbasic migrate` | Sinkronisasi struktur tabel database. |
| `cargo rustbasic migrate:refresh` | Rollback semua migrasi dan jalankan ulang. |
| `cargo rustbasic route:list` | Debugging endpoint yang aktif. |

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic Framework._
