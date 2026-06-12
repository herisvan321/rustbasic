# 🦾 AI Agent System Prompt: RustBasic Framework (SPA Edition)

## 📝 Kata Pengantar

Selamat datang di panduan **AI Agent System Prompt (SPA Edition)**. Berkas ini berfungsi sebagai instruksi operasional utama bagi asisten pengembang AI (seperti Antigravity) saat berkolaborasi membangun aplikasi di dalam ekosistem **RustBasic**. Melalui panduan ini, AI Agent diprogram untuk memahami basis pengetahuan framework, standar visual premium, langkah kerja teknis, serta metode penanganan konflik arsitektur demi menjaga kualitas kode tetap optimal.

---

## 🛠️ Script Contoh

Berikut adalah contoh skrip interaksi instruksi yang benar dari developer ke AI Agent saat meminta pembuatan halaman baru:

```text
USER: "Buat halaman kontak kami baru yang memiliki form input pesan."

AI AGENT RESPONSE (Patuh Standar):
1. Membuat rute baru di `src/routes/web.rs`:
   `.route("/contact", get(contact_controller::index))`
2. Membuat kontroller di `src/app/http/controllers/contact_controller.rs` yang mengembalikan response Inertia:
   `inertia(&req, "Contact", json!({ "title": "Hubungi Kami" }))`
3. Membuat komponen React di `src/resources/js/Pages/Contact.tsx` menggunakan hook `useForm` dari `@inertiajs/react` dan class Tailwind CSS modern.
```

---

## 🔄 Perbandingan Pemakaian (Respon AI Berbasis Prompt vs Respon AI Tanpa Konteks)

Berikut adalah perbandingan pemakaian antara AI Agent yang mengikuti system prompt ini dengan AI biasa tanpa panduan framework:

| Kriteria Respon | AI Agent Berbasis System Prompt RustBasic | AI Biasa (Tanpa Panduan Khusus) |
| :--- | :--- | :--- |
| **Arsitektur Web** | Selalu mengimplementasikan React + Inertia SPA. | Rawan menyarankan HTMX, Jinja, atau MPA tradisional. |
| **Keamanan Sesi** | Menggunakan cookie-based session terenkripsi. | Mengusulkan JWT token manual di localStorage. |
| **Gaya Penulisan** | Modular, menggunakan wrapper native core engine. | Menulis kueri raw database/driver pihak ketiga acak. |
| **Styling Tampilan**| Mengusulkan utility Tailwind premium (glassmorphism).| Menggunakan styling CSS manual atau template basic polos. |

---

## 📊 Tabel Ringkasan Basis Pengetahuan Wajib AI Agent

Berikut adalah daftar referensi berkas internal yang wajib dibaca dan dirujuk oleh AI Agent sebelum memulai tugas modifikasi kode:

| Nama File Panduan | Jalur Folder Berkas | Fokus Utama Materi Pembahasan |
| :--- | :--- | :--- |
| **`agents.md`** | `.dev/instructions/agents.md` | Alur kerja arsitektur pengiriman data dari router ke React. |
| **`inertia.md`**| `.dev/instructions/inertia.md`| Panduan Inertia routing, hook useForm, usePage, & CSRF. |
| **`catatan.md`**| `.dev/instructions/catatan.md`| Riwayat perubahan framework dan fitur keamanan inti. |
| **`README.md`**| `docs/README.md` | Indeks peta navigasi seluruh dokumen panduan resmi. |

---

## 🏁 Penutup

Dengan mematuhi instruksi sistem prompt ini secara konsisten, AI Agent dapat bertindak sebagai asisten pengembang tingkat dunia yang andal, efisien, serta secara aktif menjaga integritas arsitektur web modern monolitik RustBasic Anda.
