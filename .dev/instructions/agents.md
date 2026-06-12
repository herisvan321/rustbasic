# 🤖 RustBasic AI Agents Handbook (SPA Edition)

## 📝 Kata Pengantar

Selamat datang di **RustBasic AI Agents Handbook (SPA Edition)**. Dokumen ini bertindak sebagai panduan standar kerja operasional tertinggi bagi asisten AI (seperti Antigravity, Cursor, dll.) serta pengembang saat mengembangkan fitur, merekayasa struktur, atau memodifikasi kode di dalam proyek framework **RustBasic**. Panduan ini memastikan kepatuhan penuh pada filosofi Single Page Application (SPA) monolith, visual premium, dan embedding aset satu biner.

---

## 🛠️ Script Contoh

### A. Contoh Penulisan Rute Web (`src/routes/web.rs`)
```rust
use rustbasic_core::{Router, get, AppState};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        // Memetakan rute beranda ke method index controller
        .route("/", get(welcome_controller::index))
}
```

### B. Contoh Penulisan Controller RustBasic (`src/app/http/controllers/welcome_controller.rs`)
```rust
use crate::app::inertia::inertia;
use rustbasic_core::{Request, Response, IntoResponse, serde_json::json};

pub async fn index(req: Request) -> impl IntoResponse {
    // Mengembalikan response Inertia menuju halaman Welcome di frontend React
    inertia(&req, "Welcome", json!({
        "welcomeMessage": "Halo dari Backend RustBasic!",
        "stats": { "users": 150, "active": 90 }
    }))
}
```

### C. Contoh Penulisan Halaman React SPA (`src/resources/js/Pages/Welcome.tsx`)
```tsx
import React from 'react';
import { Link } from '@inertiajs/react';

interface WelcomeProps {
  welcomeMessage: string;
  stats: {
    users: number;
    active: number;
  };
}

export default function Welcome({ welcomeMessage, stats }: WelcomeProps) {
  return (
    <div className="min-h-screen bg-slate-950 text-white flex flex-col justify-center items-center font-sans">
      <h1 className="text-4xl font-extrabold tracking-tight mb-4">{welcomeMessage}</h1>
      <p className="text-slate-400">Pengguna Aktif: {stats.active}</p>
      {/* WAJIB menggunakan <Link> untuk navigasi internal SPA tanpa refresh */}
      <Link href="/about" className="mt-6 text-indigo-400 hover:underline">
        Tentang Aplikasi →
      </Link>
    </div>
  );
}
```

---

## 🔄 Perbandingan Pemakaian (Multi-Page Application vs Single Page Application)

Berikut adalah perbandingan pemakaian antara arsitektur web tradisional (MPA) dan arsitektur modern SPA monolith di RustBasic:

| Karakteristik Arsitektur | Multi-Page Application (MPA) | Single Page Application (SPA) |
| :--- | :--- | :--- |
| **Pemuatan Halaman** | Memuat ulang seluruh halaman (full refresh). | Hanya menukar komponen secara dinamis (bebas refresh). |
| **Bahasa Templating** | Menulis markup server template (Jinja/HTML). | Menulis komponen modern React (.tsx) di frontend. |
| **Pertukaran Data** | Mengirimkan kode HTML utuh dari server. | Hanya bertukar payload JSON mentah dari controller. |
| **Keterjagaan State** | State visual browser hilang setiap navigasi. | State global React tetap terjaga di seluruh halaman. |

---

## 📊 Tabel Ringkasan Pemetaan Folder Utama Proyek

Berikut adalah tabel pemetaan folder wajib yang harus dipatuhi oleh AI Agent saat melakukan operasi file:

| Nama Komponen Proyek | Lokasi Penyimpanan Direktori | Deskripsi Fungsi Direktori |
| :--- | :--- | :--- |
| **Controller Logika** | `src/app/http/controllers/` | Tempat menyimpan berkas logika pengolahan request RustBasic. |
| **Model Database** | `src/app/models/` | Definisi model entity database menggunakan SQLx. |
| **Rute & URL** | `src/routes/` | Berkas pendaftaran URL web (`web.rs`) & API (`api.rs`). |
| **React Halaman SPA** | `src/resources/js/Pages/` | Komponen halaman React (.tsx) yang dipetakan oleh controller. |
| **React Komponen Modular**| `src/resources/js/Components/` | Modular kecil reusable UI seperti Navbar, Sidebar, & Card. |
| **HTML Root Container** | `src/resources/views/app.rb.html` | Satu-satunya file HTML root sebagai tempat hidrasi awal React. |
| **Database Migrations** | `database/migrations/` | Berkas skema tabel database relasional. |

---

## 🏁 Penutup

Handbook panduan ini dirancang untuk menjaga keselarasan pemahaman arsitektur, kepatuhan teknis, dan standar visual premium yang ditawarkan oleh framework RustBasic. AI Agent wajib menjadikan dokumen ini sebagai instruksi operasional tertinggi saat berkolaborasi membangun sistem aplikasi Anda.
