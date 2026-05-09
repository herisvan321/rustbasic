# Dasar-Dasar RustBasic

## 🛣️ Routing
Routing dikelola di `src/routes/web.rs`. RustBasic menggunakan Axum sebagai engine utamanya.

### Mendefinisikan Route
```rust
use axum::{routing::get, Router};
use crate::app::http::controllers::welcome_controller;
use rustbasic_core::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index))
}
```

---

## ⚙️ Controllers
Controller disimpan di `src/app/http/controllers/`. Anda bisa menggunakan CLI untuk membuatnya.

### Membuat Controller
```bash
rustbasic make:controller WelcomeController
```

### Contoh Logika
```rust
use crate::app::view;
use rustbasic_core::requests::Request;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn index(req: Request) -> impl IntoResponse {
    view(&req, "welcome.rb.html", context! { 
        title => "Home" 
    })
}
```

---

## 🎨 Views
Template RustBasic menggunakan ekstensi `.rb.html` dan menggunakan sintaks **Minijinja**.

### Folder Template
Seluruh template berada di `src/resources/views/`.

---

## 📦 Asset Management
Asset inti (CSS & HTMX) dikelola oleh library `rustbasic-core`.

### Penggunaan di Layout
Gunakan helper global untuk memanggil CSS dan JS inti:
```html
<head>
    {{ app_css() | safe }}
    {{ htmx_js() | safe }}
</head>
```
