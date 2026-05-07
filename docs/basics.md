# Dasar-Dasar RustBasic

## 🛣️ Routing
Routing dikelola di `src/routes/web.rs`. RustBasic menggunakan Axum sebagai engine utamanya.

### Mendefinisikan Route
```rust
use axum::{routing::get, Router};
use crate::app::http::controllers::welcome_controller;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index))
}
```

---

## ⚙️ Controllers
Controller disimpan di `src/app/http/controllers/`. Anda bisa menggunakan CLI untuk membuatnya.

### Membuat Controller
```bash
cargo rustbasic make:controller WelcomeController
```

### Contoh Logika
```rust
pub async fn index(req: Request) -> impl IntoResponse {
    view(&req, "welcome.rb.html", context! { 
        title => "Home" 
    })
}
```

---

## 🎨 Views
Template RustBasic menggunakan ekstensi `.rb.html` dan menggunakan sintaks **Minijinja** (mirip Jinja2/Django). **Sistem komponen RSX telah dihapus** untuk mendukung performa dan kesederhanaan.

### Mewarisi Layout (Inheritance)
Gunakan tag `{% extends %}` untuk mewarisi tata letak utama:

```html
{% extends "layouts/app.rb.html" %}

{% block content %}
    <div class="card">
        <h1>Halo RustBasic!</h1>
        <p>Selamat datang di framework monolith modern.</p>
        <button class="btn btn-primary">Klik Saya</button>
    </div>
{% endblock %}
```

### Folder Template
Seluruh template berada di `src/resources/views/`.

---

## 📦 Asset Management
Asset inti (CSS & HTMX) ditanam langsung ke dalam binary aplikasi melalui sistem `rust-embed`.

### Penggunaan di Layout
Gunakan helper global untuk memanggil CSS dan JS inti:
```html
<head>
    {{ app_css() | safe }}
    {{ htmx_js() | safe }}
</head>
```

File sumber asset berada di:
- `src/resources/css/style.css`
- `src/resources/js/htmx.min.js`
