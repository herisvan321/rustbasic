# 🦾 AI Action Center: RustBasic Framework (React SPA Edition)

## 📝 Kata Pengantar

Selamat datang di **AI Action Center (React SPA Edition)**. Dokumen ini dirancang sebagai panduan aksi cepat bagi pengembang dan AI Agent untuk memahami struktur folder modular, standar wajib penulisan komponen React SPA, serta referensi instruksi lengkap penggunaan alat baris perintah (**CLI**) RustBasic. Melalui pemahaman taktis ini, proses scaffolding, manipulasi database, dan audit keamanan dapat dijalankan dengan cepat dan aman secara konsisten.

---

## 🛠️ Script Contoh

### A. Contoh Pembuatan File Boilerplate Menggunakan CLI
```bash
# 1. Menghasilkan controller baru di src/app/http/controllers/
rustbasic make:controller ArticleController

# 2. Menghasilkan model beserta migrasi database secara instan
rustbasic make:model Article -m
```

### B. Contoh Sinkronisasi Database Migrasi & Seeder
```bash
# Menjalankan seluruh berkas migrasi database
rustbasic migrate

# Mengisi database dengan data dummy awal (seeder)
rustbasic db:seed
```

### C. Menampilkan Seluruh Rute URL Aktif di Proyek Anda
```bash
# Menampilkan tabel method, path, dan handler rute web
rustbasic route:list
```

---

## 🔄 Perbandingan Pemakaian (Operasi CLI Otomatis vs Menulis Manual)

Berikut adalah perbandingan pemakaian waktu dan kepraktisan antara memanfaatkan perkakas CLI otomatis dan menulis berkas secara manual:

| Kebutuhan Tugas | Operasi Menulis Manual | Menggunakan Perintah RustBasic CLI |
| :--- | :--- | :--- |
| **Membuat File Kontroller**| Lambat, harus menyalin format import & mendesain fungsi. | Instan, template kontroller langsung terbuat dengan nama yang benar. |
| **Konfigurasi Migrasi** | Harus menghitung timestamp manual & mendaftarkan struktur.| Otomatis dibuat berkas migrasi ber-stempel waktu terkini. |
| **Menampilkan Rute Web**  | Harus melacak file routes satu per satu di editor kode. | Cukup jalankan perintah, seluruh URL & middleware tercetak rapi. |
| **Setup Autentikasi** | Memakan waktu berhari-hari untuk backend & UI React. | Instan terpasang lengkap (Breeze Scaffolding) dalam 1 detik. |

---

## 📊 Tabel Ringkasan Struktur Folder Modular Proyek

Berikut adalah peta struktur direktori modular yang wajib dipahami di dalam proyek RustBasic:

| Nama Direktori / Berkas | Fungsi & Peran Utama | Kategori Konten Berkas |
| :--- | :--- | :--- |
| **`src/app/http/controllers/`** | Pengolahan Logika Bisnis | Kontroller RustBasic yang memproses data request & props Inertia. |
| **`src/resources/js/Pages/`**   | Tampilan Halaman SPA | Komponen halaman React (.tsx) yang dirender di browser. |
| **`src/routes/web.rs`**          | Pendaftaran Jalur URL | Memetakan URL aplikasi ke controller atau closure inline. |
| **`database/migrations/`**      | Skema Struktur Tabel | Berkas migrasi blueprint pembangun skema tabel database. |
| **`storage/logs/`**             | Rekam Jejak Sistem | Berkas log harian aktivitas & tracing error backend. |
| **`.env`**                      | Konfigurasi Environment| Variabel rahasia port server, APP_KEY, & alamat database. |

---

## 🏁 Penutup

Dengan memanfaatkan **AI Action Center** dan perkakas CLI terintegrasi pada RustBasic, pengembang dapat meminimalkan kesalahan penulisan kode manual, mempercepat siklus pembuatan fitur baru, serta menjaga arsitektur aplikasi tetap bersih, rapi, dan terstandarisasi dengan baik.
