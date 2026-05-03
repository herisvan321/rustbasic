# 📑 The Complete HTMX Bible (RustBasic Monolith Edition)

Selamat datang di panduan paling komprehensif untuk **HTMX**. Dokumen ini dirancang sebagai referensi utama bagi pengembang yang bekerja dengan framework **RustBasic**. Di sini, kita akan membedah setiap sudut HTMX dari dasar hingga tingkat lanjut.

---

## 🧭 Daftar Isi
1. [Filosofi & Konsep Dasar](#1-filosofi--konsep-dasar)
2. [Referensi Atribut Lengkap](#2-referensi-atribut-lengkap)
3. [Mekanisme Trigger (Pemicu)](#3-mekanisme-trigger-pemicu)
4. [Target & Swapping (Penempatan Konten)](#4-target--swapping-penempatan-konten)
5. [Sinkronisasi & Antrean (hx-sync)](#5-sinkronisasi--antrean-hx-sync)
6. [Respon Header Server (HX-Headers)](#6-respon-header-server-hx-headers)
7. [Out-of-Band (OOB) Swaps](#7-out-of-band-oob-swaps)
8. [Event & Siklus Hidup HTMX](#8-event--siklus-hidup-htmx)
9. [Keamanan & CSRF](#9-keamanan--csrf)
10. [Integrasi dengan Rust (Axum)](#10-integrasi-dengan-rust-axum)
11. [Katalog Pola Desain (Patterns)](#11-katalog-pola-desain-patterns)
12. [Debugging & Troubleshooting](#12-debugging--troubleshooting)
13. [FAQ (Frequently Asked Questions)](#13-faq)

---

## 1. Filosofi & Konsep Dasar
HTMX berpegang pada prinsip **HATEOAS** (Hypermedia as the Engine of Application State). Artinya, server harus mengirimkan **Hypermedia (HTML)**, bukan data mentah (JSON).

### Mengapa HTMX di RustBasic?
- **Kecepatan**: Mengurangi overhead parsing JSON di klien.
- **Kesederhanaan**: Logika aplikasi tetap berada di server (Single Source of Truth).
- **Zero-JS**: Anda tidak perlu mengelola state di frontend menggunakan library seperti React atau Vue.

---

## 2. Referensi Atribut Lengkap

| Atribut | Deskripsi |
| :--- | :--- |
| `hx-get` | Mengirim AJAX GET request ke URL tertentu. |
| `hx-post` | Mengirim AJAX POST request ke URL tertentu. |
| `hx-put` | Mengirim AJAX PUT request ke URL tertentu. |
| `hx-delete` | Mengirim AJAX DELETE request ke URL tertentu. |
| `hx-patch` | Mengirim AJAX PATCH request ke URL tertentu. |
| `hx-target` | Menentukan elemen tujuan hasil request (menggunakan CSS Selector). |
| `hx-swap` | Mengatur bagaimana konten baru dimasukkan ke target. |
| `hx-trigger` | Menentukan kejadian (event) apa yang memicu request. |
| `hx-vals` | Menambahkan parameter tambahan ke request (dalam format JSON). |
| `hx-headers` | Menambahkan header HTTP khusus ke request. |
| `hx-boost` | "Meningkatkan" link dan form standar menjadi AJAX secara otomatis. |
| `hx-push-url` | Mengubah URL di address bar browser (History API). |
| `hx-indicator` | Menampilkan elemen loading saat request sedang berlangsung. |
| `hx-confirm` | Menampilkan dialog konfirmasi sebelum mengirim request. |
| `hx-prompt` | Menampilkan prompt input teks sebelum mengirim request. |
| `hx-on` | Menangani event HTMX atau browser secara inline. |
| `hx-sync` | Mengatur sinkronisasi request antara beberapa elemen. |
| `hx-params` | Menyaring parameter apa saja yang dikirim ke server. |
| `hx-include` | Memasukkan input dari elemen lain ke dalam request. |
| `hx-select` | Memilih bagian tertentu dari respon HTML server. |
| `hx-ext` | Mengaktifkan ekstensi HTMX. |
| `hx-disable` | Menonaktifkan HTMX pada elemen dan anak-anaknya. |
| `hx-validate` | Memaksa validasi HTML5 sebelum request dikirim. |

---

## 3. Mekanisme Trigger (Pemicu)
`hx-trigger` sangat fleksibel. Anda dapat mengontrol *kapan* request dikirim dengan sangat presisi.

### Trigger Modifiers:
- **`once`**: Request hanya dikirim satu kali.
- **`changed`**: Request hanya dikirim jika nilai input telah berubah.
- **`delay:<time>`**: Menunggu waktu tertentu sebelum mengirim (misal: `delay:500ms`).
- **`throttle:<time>`**: Membatasi frekuensi request (misal: `throttle:1s`).
- **`from:<selector>`**: Mendengarkan event dari elemen lain.
- **`target:<selector>`**: Hanya picu jika target event adalah selector tertentu.
- **`consume`**: Menghentikan event bubbling ke elemen induk.

### Contoh Kasus:
```html
<!-- Picu saat user berhenti mengetik selama 500ms -->
<input hx-get="/search" hx-trigger="keyup changed delay:500ms">

<!-- Picu saat elemen muncul di layar (Lazy Loading) -->
<div hx-get="/lazy-content" hx-trigger="revealed"></div>

<!-- Picu saat window di-resize -->
<div hx-get="/update-layout" hx-trigger="resize from:window"></div>
```

---

## 4. Target & Swapping (Penempatan Konten)
`hx-swap` menentukan pengalaman transisi konten.

### Nilai `hx-swap`:
- **`innerHTML`**: Mengganti konten di dalam elemen target.
- **`outerHTML`**: Mengganti seluruh elemen target dengan respon.
- **`beforebegin`**: Memasukkan respon sebelum elemen target.
- **`afterbegin`**: Memasukkan respon sebagai anak pertama elemen target.
- **`beforeend`**: Memasukkan respon sebagai anak terakhir elemen target.
- **`afterend`**: Memasukkan respon tepat setelah elemen target.
- **`delete`**: Menghapus target tanpa mempedulikan isi respon.
- **`none`**: Jangan lakukan swap (biasanya digunakan jika hanya ingin memicu `HX-Trigger` header).

### Swap Modifiers:
- **`swap:<time>`**: Memberi jeda waktu sebelum swap dilakukan (untuk animasi).
- **`settle:<time>`**: Memberi jeda waktu setelah swap (untuk transisi CSS).
- **`scroll:<direction>`**: Scroll ke `top` atau `bottom` target setelah swap.
- **`show:<direction>`**: Memaksa browser menampilkan target (focus).

---

## 5. Sinkronisasi & Antrean (hx-sync)
Gunakan `hx-sync` untuk menghindari konflik antar request.

```html
<!-- Jika form dikirim, batalkan semua request lain di form ini -->
<form hx-post="/save" hx-sync="this:replace">
    ...
</form>

<!-- Antrekan request (queue) agar dijalankan berurutan -->
<input hx-get="/validate" hx-sync="closest form:queue">
```

---

## 6. Respon Header Server (HX-Headers)
Server Rust (Axum) Anda dapat berkomunikasi balik dengan HTMX melalui header HTTP.

| Header | Deskripsi |
| :--- | :--- |
| `HX-Trigger` | Memicu event di sisi klien (bisa berupa string atau JSON). |
| `HX-Redirect` | Melakukan client-side redirect ke URL baru. |
| `HX-Refresh` | Melakukan full reload pada halaman. |
| `HX-Push-Url` | Mendorong URL baru ke dalam browser history. |
| `HX-Retarget` | Mengubah elemen target untuk swap saat itu juga. |
| `HX-Reswap` | Mengubah cara swap dilakukan (misal: dari innerHTML ke outerHTML). |

### Contoh Penggunaan di Rust:
```rust
// Memicu refresh halaman dari server
(
    [("HX-Refresh", "true")],
    "Update Berhasil"
)
```

---

## 7. Out-of-Band (OOB) Swaps
Fitur terkuat HTMX untuk mengupdate banyak elemen sekaligus.

Jika server mengembalikan:
```html
<!-- Konten Utama (Target Default) -->
<div class="main-content">Data Terbaru</div>

<!-- Konten Tambahan (Update tempat lain) -->
<div id="sidebar-info" hx-swap-oob="true">
    Info Sidebar Terupdate
</div>
```
HTMX akan otomatis mencari elemen dengan ID `sidebar-info` di manapun posisinya dan menggantinya dengan konten di atas, sementara konten utama tetap masuk ke target aslinya.

---

## 8. Event & Siklus Hidup HTMX
HTMX memancarkan banyak event yang bisa Anda dengarkan:

- `htmx:beforeRequest`: Sebelum AJAX dikirim.
- `htmx:afterRequest`: Setelah respon diterima (sukses atau gagal).
- `htmx:beforeSwap`: Sebelum konten ditukar.
- `htmx:afterSwap`: Setelah konten ditukar.
- `htmx:configRequest`: Tempat terbaik untuk memodifikasi request (misal: tambah header).
- `htmx:responseError`: Terjadi jika server mengembalikan status 4xx atau 5xx.

---

## 9. Keamanan & CSRF
Keamanan adalah prioritas di RustBasic.

### Penanganan CSRF:
HTMX mendukung `hx-headers` secara global atau per elemen. Di RustBasic, kita biasanya mengirimkan token lewat body form atau header:
```html
<form hx-post="/update" hx-headers='{"X-CSRF-Token": "{{ csrf_token }}"}'>
    ...
</form>
```

---

## 10. Integrasi dengan Rust (Axum)
Axum sangat cocok dengan HTMX karena performanya yang tinggi.

### Deteksi Request HTMX:
```rust
use axum::http::HeaderMap;

pub async fn handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        // Return partial HTML (fragment)
    } else {
        // Return full page layout
    }
}
```

---

## 11. Katalog Pola Desain (Patterns)

### 11.1 Active Search (Pencarian Langsung)
Sangat berguna untuk tabel atau daftar data.
```html
<input type="search" 
       name="search" 
       placeholder="Cari data..." 
       hx-get="/api/search" 
       hx-trigger="keyup changed delay:500ms, search" 
       hx-target="#search-results" 
       hx-indicator="#search-loading">

<div id="search-loading" class="htmx-indicator">Mencari...</div>
<div id="search-results">Hasil akan muncul di sini</div>
```

### 11.2 Click-to-Edit
Mengubah tampilan statis menjadi form input.
```html
<div id="contact-1" hx-target="this" hx-swap="outerHTML">
    <div>Nama: John Doe</div>
    <button hx-get="/contact/1/edit">Edit</button>
</div>

<!-- Server mengembalikan (saat Edit diklik): -->
<form hx-put="/contact/1" hx-target="this" hx-swap="outerHTML">
    <input name="name" value="John Doe">
    <button type="submit">Simpan</button>
    <button hx-get="/contact/1">Batal</button>
</form>
```

### 11.3 Bulk Update (Checkboxes)
```html
<form hx-post="/users/activate" hx-target="#user-table">
    <table id="user-table">
        <tr>
            <td><input type="checkbox" name="ids" value="1"></td>
            <td>User 1</td>
        </tr>
        <tr>
            <td><input type="checkbox" name="ids" value="2"></td>
            <td>User 2</td>
        </tr>
    </table>
    <button type="submit">Aktifkan Terpilih</button>
</form>
```

### 11.4 Infinite Scroll
```html
<div id="post-list">
    <!-- Item 1..10 -->
    <div hx-get="/posts?page=2" 
         hx-trigger="revealed" 
         hx-swap="afterend" 
         hx-indicator="#loading">
        Picu muat data selanjutnya...
    </div>
</div>
<div id="loading" class="htmx-indicator">Memuat lebih banyak...</div>
```

### 11.5 Lazy Loading
Memuat bagian berat (seperti chart atau statistik) setelah halaman utama muncul.
```html
<div hx-get="/charts/stats" hx-trigger="load">
    <div class="skeleton-loader">Sedang memuat data grafik...</div>
</div>
```

---

## 12. Debugging & Troubleshooting

### Tips Debugging:
1.  **`htmx.logAll()`**: Jalankan di konsol browser untuk melihat log detail setiap aksi.
2.  **Chrome DevTools**: Periksa tab `Network`. Klik pada request dan lihat tab `Response`. Jika isinya adalah seluruh halaman HTML padahal Anda hanya butuh fragment, berarti ada yang salah di sisi server (Axum).
3.  **Visual Debugging**: Tambahkan CSS berikut untuk melihat elemen mana yang sedang loading:
    ```css
    .htmx-request {
        opacity: 0.5;
        transition: opacity 200ms ease-in;
    }
    ```

### Masalah Umum:
- **Respon 404/405**: Periksa kembali URL dan Method di `src/routes/web.rs`.
- **Konten tidak berubah**: Pastikan ID di `hx-target` sesuai dengan ID di HTML Anda.
- **CSRF Error**: Pastikan token dikirim dengan nama field yang benar (biasanya `csrf_token`).

---

## 13. FAQ

**T: Apakah HTMX aman untuk SEO?**
J: Ya, jika Anda menggunakan `hx-boost`. Mesin pencari akan melihat link standar, sementara pengguna akan mendapatkan pengalaman AJAX yang cepat.

**T: Kapan saya harus tetap menggunakan JavaScript manual?**
J: Gunakan JS manual (atau library seperti Hyperscript/Alpine) hanya untuk logika murni frontend yang tidak butuh data server, seperti toggle class, animasi kompleks, atau manipulasi DOM yang bersifat instan (seperti canvas).

**T: Bisakah HTMX bekerja dengan WebSockets?**
J: Sangat bisa. HTMX memiliki ekstensi `ws` yang sangat handal untuk komunikasi dua arah real-time.

---

_Dokumentasi ini adalah panduan hidup. Teruslah bereksperimen dengan HTMX untuk menciptakan aplikasi Rust yang luar biasa cepat dan efisien!_

---

## Tambahan: Tips Macro MiniJinja untuk HTMX

Di RustBasic, kita sangat menyarankan penggunaan macro untuk menjaga konsistensi:

```html
{% macro htmx_button(label, action, target, method="post") %}
    <button hx-{{ method }}="{{ action }}" 
            hx-target="{{ target }}" 
            hx-indicator="#indicator"
            class="btn-premium">
        {{ label }}
    </button>
{% endmacro %}
```

Gunakan di template Anda:
```html
{{ htmx_button("Simpan Profile", "/profile/update", "#profile-card") }}
```

---

## Glosarium Istilah Penting

- **HATEOAS**: Hypermedia as the Engine of Application State.
- **Polymorphic Fragments**: Mengirimkan bagian HTML yang berbeda-beda tergantung konteks request.
- **OOB**: Out of Band (penggantian elemen di luar target utama).
- **Settling**: Proses browser menerapkan style CSS setelah elemen baru dimasukkan ke DOM.
- **Boosting**: Proses otomatisasi AJAX pada elemen anchor dan form tradisional.

---

## Sejarah Singkat
HTMX adalah evolusi dari intercooler.js, diciptakan untuk membawa kembali kemudahan pengembangan web era 90-an namun dengan kapabilitas browser modern tahun 2020-an.

---

## Sumber Daya Tambahan
- Situs Resmi: [htmx.org](https://htmx.org)
- Dokumentasi Referensi: [htmx.org/reference/](https://htmx.org/reference/)
- Contoh Lanjutan: [htmx.org/examples/](https://htmx.org/examples/)

---

_Catatan: Jika Anda menemukan bug atau ingin menambahkan pola desain baru ke dokumen ini, silakan hubungi tim pengembang RustBasic._
