# 📑 The Complete Inertia.js & React SPA Bible (RustBasic Monolith Edition)

## 📝 Kata Pengantar

Selamat datang di **Inertia.js & React SPA Bible (RustBasic Monolith Edition)**. Dokumentasi ini merupakan panduan tertinggi bagi pengembang dan AI Agent untuk memahami konsep, mekanisme kerja backend RustBasic, interaktivitas React, pengolahan form data reaktif (`useForm`), serta penggunaan shared props global (`usePage`) di dalam ekosistem **RustBasic**. Melalui pemahaman mendalam tentang jembatan Inertia, Anda dapat membangun aplikasi web monolith modern yang interaktif, super kencang, dan bebas reload.

---

## 🛠️ Script Contoh

### A. Contoh Controller Backend RustBasic Mengirim Render Inertia
```rust
use crate::app::inertia::inertia;
use rustbasic_core::requests::Request;
use rustbasic_core::Response;
use rustbasic_core::serde_json::json;

pub async fn show_profile(req: Request) -> Response {
    // Mengembalikan response Inertia ke halaman React "Dashboard/Profile"
    inertia(req, "Dashboard/Profile", json!({
        "username": "Heris Van Hendra",
        "email": "heris@example.com",
        "role": "Superadmin"
    }))
}
```

### B. Contoh Form Submission Menggunakan Hook `useForm` di React
```tsx
import { useForm } from '@inertiajs/react';
import React, { FormEvent } from 'react';

export default function EditProfile() {
  const { data, setData, post, processing, errors } = useForm({
    name: '',
  });

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    post('/profile/update'); // Kirim form via POST request latar belakang
  };

  return (
    <form onSubmit={handleSubmit} className="p-6 bg-slate-900 rounded-xl">
      <input type="text" value={data.name} onChange={e => setData('name', e.target.value)} className="bg-slate-950 p-2 text-white" />
      {errors.name && <div className="text-red-500 text-xs">{errors.name}</div>}
      <button type="submit" disabled={processing} className="mt-2 px-4 py-2 bg-indigo-600 rounded">
        {processing ? 'Menyimpan...' : 'Simpan'}
      </button>
    </form>
  );
}
```

### C. Contoh Membaca Flash Message Global Menggunakan Hook `usePage`
```tsx
import { usePage } from '@inertiajs/react';
import React from 'react';

interface PageProps {
  flash: {
    success?: string;
    error?: string;
  };
  [key: string]: any;
}

export default function Header() {
  const { props } = usePage<PageProps>();
  const { flash } = props; // flash message disuplai otomatis dari backend

  return (
    <header className="p-4 bg-slate-950 flex justify-between text-white">
      <span>RustBasic Project</span>
      {flash.success && <span className="text-green-400 font-bold">{flash.success}</span>}
    </header>
  );
}
```

---

## 🔄 Perbandingan Pemakaian (Redirect 302 vs Redirect 303 untuk Browser SPA)

Berikut adalah tabel perbandingan pemakaian kode status HTTP redirect dalam siklus navigasi Inertia SPA:

| Kriteria Evaluasi | HTTP Redirect 302 (Found) | HTTP Redirect 303 (See Other) |
| :--- | :--- | :--- |
| **Pola Perilaku** | Browser tetap mempertahankan metode request lama (POST/PUT/DELETE). | Browser diinstruksikan mengubah metode request menjadi GET. |
| **Respon Inertia** | Mengalami kegagalan hidrasi/redirection karena method bentrok. | Berhasil mengalihkan halaman browser SPA dan memuat data baru. |
| **Kasus Penggunaan**| Redirect rute statis reguler di luar submit AJAX. | Redirect wajib setelah formulir disubmit (POST/PUT/DELETE). |

---

## 📊 Tabel Ringkasan Komponen Utama Jembatan Inertia

Berikut adalah ringkasan utilitas dan fungsi bawaan Inertia untuk navigasi dan komunikasi stateful:

| Nama Utilitas | Jenis Elemen / Hook | Peran & Deskripsi Fungsi |
| :--- | :--- | :--- |
| **`<Link>`** | Komponen React | Navigasi bebas reload penuh (mengirim request AJAX di latar belakang). |
| **`useForm`** | React Hook | Mengelola status formulir, error validasi, dan loading state secara terpadu. |
| **`usePage`** | React Hook | Membaca data shared props global (user login, notifikasi flash, CSRF). |
| **`data-page`**| Atribut HTML | Kontainer awal JSON data untuk hidrasi React di `app.rb.html`. |
| **`CSRF token`**| HTTP Header | Proteksi keamanan otomatis lintas situs yang dikirim oleh Inertia. |

---

## 🏁 Penutup

Dengan menggabungkan perlindungan sesi berbasis cookie HTTP-only, token CSRF terautomasi, serta hook reaktif seperti `useForm` dan `usePage`, Inertia.js menyajikan kemudahan pengembangan web monolith secepat kilat dengan tingkat kenyamanan dan keamanan kelas atas.
