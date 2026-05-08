# 📄 Product Requirements Document (PRD) Prompt

Gunakan template ini untuk mendefinisikan fitur atau aplikasi baru yang akan dibangun di atas framework **RustBasic**. Masukkan detail aplikasi Anda di bawah setiap section.

---

## 🚀 1. Ringkasan Proyek (Project Overview)
*Deskripsikan secara singkat aplikasi apa yang ingin dibuat.*
- **Nama Aplikasi**: 
- **Tujuan Utama**: 
- **Target Pengguna**: 

---

## 🛠️ 2. Fitur Utama (Core Features)
*Daftar fungsionalitas yang harus ada.*
- [ ] **Fitur A**: Deskripsi cara kerja.
- [ ] **Fitur B**: Deskripsi cara kerja.
- [ ] **Auth System**: (Misal: Login/Register dengan role Admin & User).

---

## 📊 3. Struktur Data (Data Model)
*Definisikan entitas yang dibutuhkan untuk Sea-ORM.*
- **Table Name**: `users`
    - Fields: `id`, `username`, `email`, `password`, `created_at`.
- **Table Name**: `...`
    - Fields: `...`

---

## 🎨 4. Antarmuka Pengguna (UI/UX Requirements)
*Sesuai dengan Golden Rules RustBasic: Modern, Premium, Glassmorphism.*
- **Tema Warna**: (Default: Dark Mode / Glassmorphism).
- **Layout**: (Default: Split-screen untuk Auth, Sidebar untuk Dashboard).
- **Interaksi**: Menggunakan HTMX untuk:
    - [ ] Form Submission tanpa refresh.
    - [ ] Loading state (hx-indicator).
    - [ ] Modal/Dynamic Content.

---

## 🛣️ 5. Alur Pengguna & Routing (User Flow)
*Daftar endpoint yang direncanakan.*
- `GET /`: Landing page.
- `GET /dashboard`: Halaman utama setelah login.
- `POST /api/action`: Endpoint HTMX untuk...

---

## ⚠️ 6. Batasan Teknis (Technical Constraints)
*Wajib diikuti oleh AI Agent:*
1. **Frontend**: WAJIB HTML + Minijinja (`.rb.html`). DILARANG pakai RSX/Component Magic.
2. **Interaktivitas**: WAJIB HTMX. DILARANG menambah library JS berat.
3. **Styling**: WAJIB Pure CSS di `style.css` (Gunakan variabel CSS).
4. **Database**: WAJIB Sea-ORM via `cargo rustbasic migrate`.

---

> **Instruksi untuk AI**: 
> "Berdasarkan PRD di atas, buatkan rencana implementasi langkah demi langkah (Routing -> Model -> Controller -> View) dengan mengikuti aturan di `agents.md`."
