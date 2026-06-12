# 📄 Product Requirements Document (PRD) Prompt (React SPA Edition)

## 📝 Kata Pengantar

Selamat datang di template **Product Requirements Document (PRD) Prompt (React SPA Edition)**. Dokumen ini digunakan sebagai cetak biru (blueprint) awal bagi pengembang dan AI Agent untuk merencanakan pembangunan fitur atau aplikasi baru di atas framework **RustBasic**. Melalui penulisan PRD yang terstruktur, seluruh cakupan fitur, desain database, visual UI, alur navigasi, dan batasan teknis dapat diselaraskan sejak awal sebelum penulisan kode dimulai.

---

## 🛠️ Script Contoh

Berikut adalah contoh draf pengisian PRD singkat untuk fitur **Sistem Manajemen Kontak (Contact Management)**:

```markdown
### 🚀 1. Ringkasan Proyek
*   **Nama Aplikasi**: ContactBook SPA
*   **Tujuan Utama**: Mempermudah pengguna menyimpan dan melihat daftar kontak secara cepat.

### 🛠️ 2. Fitur Utama
*   [x] CRUD data kontak (Nama, Telepon, Email).
*   [x] Pencarian kontak real-time di frontend.

### 📊 3. Struktur Data (SQLx)
*   **Tabel**: `contacts`
    *   Fields: `id (int)`, `name (varchar)`, `phone (varchar)`, `email (varchar)`.

### 🎨 4. Antarmuka Pengguna (UI/UX)
*   Menggunakan layout Bento Grid, tema default dark mode, dan form submission instan via `useForm` Inertia helper.

### 🛣️ 5. Alur Pengguna & Routing
*   `GET /contacts` -> Merender halaman `Contacts/Index.tsx`
*   `POST /contacts` -> Menyimpan data kontak baru di backend
```

---

## 🔄 Perbandingan Pemakaian (Pengembangan Berbasis PRD vs Pengembangan Ad-Hoc)

Berikut adalah perbandingan pemakaian metode perencanaan dalam pengembangan fitur web SPA:

| Aspek Pengembangan | Pengembangan Berbasis PRD Terstruktur | Pengembangan Ad-Hoc (Tanpa Rencana) |
| :--- | :--- | :--- |
| **Kejelasan Struktur Data**| Skema database terdefinisi matang di awal. | Struktur tabel sering diubah di tengah jalan. |
| **Konsistensi Alur URL** | Peta rute web terdokumentasi dengan jelas. | Rawan terjadi tumpang tindih atau konflik rute URL. |
| **Kesesuaian Desain UI**  | Desain Tailwind dan transisi SPA terencana rapi. | Tampilan tidak konsisten antar halaman. |
| **Efisiensi Debugging** | Bug mudah dilacak sesuai batasan teknis PRD. | Membuang waktu mencari letak error karena alur acak. |

---

## 📊 Tabel Ringkasan Komponen Wajib Dokumen PRD

Berikut adalah ringkasan parameter utama yang wajib diisi dalam setiap dokumen PRD:

| Parameter PRD | Fungsi Isian | Contoh Kebutuhan Data |
| :--- | :--- | :--- |
| **Ringkasan Proyek** | Mendefinisikan visi dan target aplikasi. | Nama aplikasi, tujuan utama, & segmen user. |
| **Fitur Utama** | Menyusun daftar checklist fitur prioritas. | Sistem autentikasi, dashboard panel, CRUD produk. |
| **Struktur Data** | Merancang skema tabel database (SQLx). | Nama kolom, tipe data, relasi foreign key. |
| **Antarmuka (UI/UX)** | Menetapkan tema dan interaktivitas SPA. | Dark mode, form helper `useForm`, layout bento. |
| **Alur Pengguna** | Memetakan URL rute web dan responsnya. | `GET /profile`, `POST /profile/update`. |
| **Batasan Teknis** | Menegaskan batasan teknologi yang wajib dipakai. | Komponen React `.tsx`, Tailwind styling, single biner build. |

---

## 🏁 Penutup

Menggunakan dokumen PRD sebagai acuan kerja membantu menjaga integritas arsitektur aplikasi web SPA Anda, menyelaraskan persepsi antar anggota tim, serta memastikan kualitas akhir produk yang dibuat dengan framework RustBasic tetap premium dan berkinerja tinggi.
