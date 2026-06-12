# 🎨 RustBasic AI Template Workflow (React SPA Edition)

## 📝 Kata Pengantar

Selamat datang di panduan **AI Template Workflow (React SPA Edition)**. Dokumen ini mendefinisikan alur kerja standar bagi AI Agent dan pengembang saat melakukan konversi atau ekspor aset visual dari berkas HTML statis (seperti `template.html`) ke dalam proyek berbasis **React.js + Inertia.js** pada framework **RustBasic**. Panduan ini memandu Anda memecah kerangka template statis ke dalam berkas-berkas halaman inti React (TypeScript), berkas CSS proyek, komponen modular, serta pendaftaran rute backend secara terstruktur dan terstandarisasi.

---

## 📁 Nama Project & Aturan Ekspor Struktur Project

Sebelum melakukan ekspor dari `template.html`, Anda wajib menentukan **Nama Project** dan **Nama Template** yang akan diekspor (misal: `nama-template`). Nama template ini akan digunakan untuk menghasilkan berkas controller, halaman React (TypeScript), serta file stylesheet yang mewakili file inti tersebut di dalam struktur proyek.

Berikut adalah peta struktur direktori proyek `<nama-project>` yang dihasilkan berdasarkan penamaan template `<nama-template>` yang diekspor:

```text
<nama-project>/
├── Cargo.toml                  # Nama project mewakili nama crate inti
├── package.json                # Konfigurasi dependensi npm frontend
├── tsconfig.json               # Konfigurasi TypeScript compiler
├── vite.config.js              # Konfigurasi bundler Vite untuk SPA
├── src/
│   ├── main.rs                 # Berkas utama inisialisasi server RustBasic
│   ├── app/
│   │   ├── http/
│   │   │   └── controllers/    # Berkas kontroller (misal: <nama_template>_controller.rs)
│   │   └── inertia.rs          # Helper rendering jembatan Inertia
│   └── routes/
│       └── web.rs              # Pendaftaran rute web & import controller <nama_template>
├── src/resources/
│   ├── js/
│   │   ├── Pages/              # Berkas halaman React utama (TypeScript, misal: <NamaTemplate>.tsx)
│   │   ├── Components/         # Komponen modular pembantu visual terkait (TypeScript, misal: <NamaTemplate>Navbar.tsx)
│   │   └── main.tsx            # Entry point rendering frontend React + TypeScript
│   └── css/
│       └── style.css           # Berkas stylesheet CSS utama proyek (termasuk directives Tailwind CSS)
└── public/
    └── assets/                 # Folder penyimpanan gambar & file statis
```

---

## 🛠️ Script Contoh

### A. Contoh Ekstraksi CSS dari `template.html` ke Proyek (`src/resources/css/style.css`)
```css
/* Ekspor class styling khusus dari template.html ke folder css proyek */
@tailwind base;
@tailwind components;
@tailwind utilities;

.glassmorphism {
  background: rgba(15, 23, 42, 0.6);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.05);
}
```

### B. Contoh Pembuatan Berkas Halaman Utama React SPA (`src/resources/js/Pages/Welcome.tsx`)
```tsx
import React from 'react';
import { Link } from '@inertiajs/react';
import Navbar from '../Components/Navbar';

interface WelcomeProps {
  title: string;
  appVersion: string;
}

export default function Welcome({ title, appVersion }: WelcomeProps) {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 font-sans">
      <Navbar />
      <main className="max-w-4xl mx-auto mt-12 p-8 glassmorphism rounded-2xl">
        <h1 className="text-4xl font-extrabold tracking-tight mb-4">{title}</h1>
        <p className="text-slate-400 mb-6">Aplikasi berjalan pada versi: {appVersion}</p>
        <Link href="/dashboard" className="px-6 py-3 bg-indigo-600 rounded-xl hover:bg-indigo-500 font-semibold transition-colors duration-200">
          Masuk ke Dashboard
        </Link>
      </main>
    </div>
  );
}
```

### C. Mendaftarkan Rute & Controller Baru (`src/routes/web.rs`)
```rust
use rustbasic_core::{Router, get, AppState, Request, Response, IntoResponse};
use rustbasic_core::serde_json::json;
use crate::app::inertia::inertia;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(|req: Request| async move {
            inertia(&req, "Welcome", json!({
                "title": "Welcome to RustBasic SPA",
                "appVersion": "2026.1"
            }))
        }))
}
```

---

## 🔄 Perbandingan Pemakaian (Direct HTML Rendering vs React-Inertia SPA Component)

Berikut adalah perbandingan pemakaian antara menggunakan visual HTML statis langsung dan mengekspornya ke komponen SPA React-Inertia:

| Karakteristik | Direct HTML Rendering (`template.html`) | React-Inertia SPA Component (`Welcome.tsx`) |
| :--- | :--- | :--- |
| **Kecepatan Navigasi** | Memicu refresh halaman penuh yang lambat. | Navigasi instan bebas reload (AJAX swap). |
| **Modul & Reusability** | Sulit memisahkan komponen berulang (Navbar, Footer).| Sangat mudah memisahkannya ke komponen re-usable. |
| **Manajemen State** | Menggunakan variabel DOM global/JQuery yang rawan konflik.| Terkelola rapi menggunakan state internal React. |
| **Konektivitas Backend**| Harus menulis API request manual (Fetch/Axios). | Otomatis tersambung via *Props* controller backend. |

---

## 📊 Tabel Ringkasan Aturan Ekspor Berkas dari `template.html`

Saat memindahkan desain dari berkas HTML mentah (`template.html`), gunakan tabel rujukan pemetaan nama berkas dan folder berikut untuk mewakili komponen proyek Anda:

| Elemen Asal (`template.html`) | Berkas Hasil Ekspor | Direktori Tujuan Ekspor | Deskripsi Fungsi |
| :--- | :--- | :--- | :--- |
| **Body Utama & Layout** | `Welcome.tsx` | `src/resources/js/Pages/` | Menjadi berkas halaman inti visual SPA. |
| **CSS internal / external** | `style.css` | `src/resources/css/` | Berisi styling global & utility Tailwind. |
| **Navbar & Sidebar** | `Navbar.tsx` | `src/resources/js/Components/`| Komponen modular navigasi terpisah. |
| **Image & static assets** | `logo.png` | `public/assets/` | File statis yang diakses langsung dari web server. |

---

## 🏁 Penutup

Dengan mengikuti alur konversi terstandar ini, pemindahan visual dari berkas HTML statis seperti `template.html` ke dalam arsitektur React-Inertia SPA dapat dilakukan dengan rapi, efisien, dan tetap menjaga keterbacaan kode sistem Anda secara optimal.
