# 🎨 Panduan View & Template

RustBasic menggunakan **Minijinja** sebagai mesin template yang sangat cepat dan mirip dengan Jinja2/Django. Seluruh file template menggunakan ekstensi `.rb.html`.

## 🚀 Sintaks Template Dasar
Anda dapat menulis HTML standar secara langsung. Tidak ada tag kustom ajaib.

### 1. Menampilkan Variabel
Gunakan tanda kurung kurawal ganda:
```html
<h1>Selamat Datang, {{ nama_user }}</h1>
```

### 2. Struktur Kontrol (If / For)
Gunakan tag blok Minijinja standar:
```html
{% if is_logged_in %}
    <p>Halo Admin!</p>
{% else %}
    <p>Silakan Login.</p>
{% endif %}

<ul>
{% for item in items %}
    <li>{{ item }}</li>
{% endfor %}
</ul>
```

### 3. Layouts & Inheritance
Gunakan ekstensi untuk mewarisi tata letak utama (`app.rb.html`):
```html
{% extends "layouts/app.rb.html" %}

{% block title %}Halaman Dashboard{% endblock %}

{% block content %}
    <div>Konten Halaman</div>
{% endblock %}
```

---

## 🧩 Sistem Desain (Pure CSS & HTML)
Daripada menggunakan sistem komponen kustom (seperti `<Buttons.Button />`), framework ini mendorong penggunaan kelas utilitas CSS murni (seperti Tailwind/Bootstrap) langsung pada elemen HTML.

### 1. Buttons
```html
<button class="btn btn-primary">Simpan</button>
<a href="/login" class="btn btn-outline">Batal</a>
```

### 2. Forms
```html
<label class="form-label">Email</label>
<input type="email" name="email" class="form-control" placeholder="nama@email.com">
```

### 3. Alerts
```html
<div class="alert alert-success">Berhasil disimpan!</div>
```

---

## 📅 Filter Waktu & Tanggal (Carbon-like)
Anda tetap dapat menggunakan filter Minijinja bawaan RustBasic:
1. **`diff_for_humans`**: `{{ user.created_at | diff_for_humans }}` -> *"2 hours ago"*
2. **`format_date`**: `{{ now() | format_date("%d %B %Y") }}` -> *"02 May 2026"*
3. **`now()`**: Mendapatkan waktu saat ini.

---

## 🛡️ Keamanan & Privasi Source Code
Secara default, RustBasic melakukan **Minifikasi Otomatis** pada output HTML. Saat pengguna melakukan "View Source" di browser:
- Semua spasi berlebih dan baris baru dihapus.
- Semua komentar HTML (`<!-- ... -->`) dibuang.
- Kode akan tampak sebagai satu baris rapat yang sulit dibaca (Obfuscation ringan).

---

## 📦 Template Embedding (rust-embed)
Secara default, RustBasic menggunakan sistem **Hybrid Loading** untuk efisiensi maksimal:

1.  **Mode Debug (Development)**: Template dibaca langsung dari folder `src/resources/views/`. Ini memungkinkan fitur **Live Reload** bekerja tanpa perlu kompilasi ulang setiap kali ada perubahan file `.rb.html`.
2.  **Mode Release (Production)**: Seluruh file template di-embed ke dalam file binary menggunakan `rust-embed`. Hal ini membuat aplikasi Anda menjadi satu file executable tunggal yang portabel, lebih cepat, dan tidak lagi membutuhkan folder `src/resources/views/` di server produksi.

---

## 🔄 Hot Reload & Pengembangan
Gunakan perintah berikut untuk pengembangan yang super cepat dengan auto-refresh browser:
```bash
cargo serve
```
Setiap kali file `.rb.html` di `src/resources/views/` disimpan, browser akan otomatis memuat ulang halaman.
