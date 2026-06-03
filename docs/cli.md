# 🛠️ Panduan Perintah CLI RustBasic

## 📝 Kata Pengantar
Selamat datang di panduan resmi **RustBasic CLI**. Dokumentasi ini dirancang khusus untuk memandu Anda menguasai baris perintah (Command Line) bawaan framework. Dengan menguasai perkakas CLI ini, Anda dapat mengotomatisasi pembuatan struktur folder, file kode boilerplate (generator), pengelolaan migrasi database, hingga audit performa dan keamanan secara instan di terminal Anda.

---

## 🛠️ Script Contoh

### A. Membuat Proyek Baru
```bash
rustbasic new my-new-app
```

### B. Membuat Controller Baru di Terminal
```bash
rustbasic make:controller ArticleController
```

### C. Menampilkan Daftar Rute yang Aktif
```bash
rustbasic route:list
```

### D. Memulai Proses Build Produksi
```bash
rustbasic build
```

---

## 🔄 Perbandingan Pemakaian Perintah Lokal vs Perintah Global

Berikut adalah perbandingan pemakaian arsitektur delegasi perintah CLI pada RustBasic:

| Kriteria Perbandingan | Perintah CLI Global (`rustbasic-cli`) | Perintah CLI Lokal (`rustbasic <cmd>`) |
| :--- | :--- | :--- |
| **Metode Instalasi** | Terinstal secara global di OS (`cargo install`). | Dijalankan langsung di folder root proyek lokal Anda. |
| **Tugas Utama** | Membuat kerangka folder proyek baru (`new`). | Menjalankan migrasi, membuat model, controller, & seeder. |
| **Sinkronisasi Versi** | Independen dari versi library proyek. | Otomatis selaras dengan versi dependensi lokal proyek Anda. |
| **Sumber Eksekusi** | File biner global system. | Didelegasikan ke file biner proyek (`src/config/cli.rs`). |

---

## 📊 Tabel Ringkasan Pintasan CLI Lengkap

Berikut adalah daftar perintah baris teks yang sering digunakan dalam proses pengembangan:

| Perintah CLI | Deskripsi Singkat Perintah | Output / Hasil Eksekusi |
| :--- | :--- | :--- |
| **`make:controller`** | Membuat berkas kontroler baru. | Berkas baru di `src/app/http/controllers/`. |
| **`make:model -m`** | Membuat model beserta migrasinya. | Berkas model di `src/app/models/` & berkas migrasi baru. |
| **`make:middleware`**| Membuat berkas middleware baru. | Berkas baru di `src/app/http/middleware/`. |
| **`key:generate`** | Membuat kunci enkripsi APP_KEY baru. | Kunci base64 baru tersimpan otomatis di berkas `.env`. |
| **`storage:link`** | Menghubungkan folder privat ke publik. | Folder `public/storage` terhubung ke `storage/app/public`. |
| **`route:list`** | Menampilkan daftar rute dan middleware.| Tabel rute tercetak rapi di terminal Anda. |
| **`build`** | Mengompilasi kode program Rust & aset Frontend. | Folder `deploy/` yang berisi biner executable & aset teroptimasi. |

---

## 🏁 Penutup
Perkakas CLI pada RustBasic dirancang untuk menghilangkan kejenuhan menulis kode boilerplate secara manual. Dengan memanfaatkan pintasan generator ini, Anda dapat fokus sepenuhnya pada pembangunan logika bisnis utama aplikasi.
