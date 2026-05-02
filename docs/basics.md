# Routing

Routing di RustBasic dikelola di dalam folder `src/routes/`. Utama-nya berada di `src/routes/web.rs`.

## Mendefinisikan Route
Anda dapat mendefinisikan route menggunakan Axum syntax:

```rust
use axum::{routing::{get, post}, Router};
use crate::app::http::controllers::WelcomeController;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(WelcomeController::index))
        // Tambahkan route lainnya di sini
}
```

## Route Grouping & Prefix
Gunakan `.nest()` untuk mengelompokkan route:

```rust
Router::new()
    .nest("/admin", admin_routes())
```

---

# Controllers

Controller bertugas menangani logika request dan mengembalikan response. Disimpan di `src/app/http/controllers/`.

## Membuat Controller
Gunakan CLI yang sudah mendukung warna dan tabel:
```bash
cargo rustbasic make:controller Name
cargo rustbasic make:middleware Name
```

## Melihat Daftar Route
Gunakan perintah berikut untuk melihat seluruh URL yang terdaftar:
```bash
cargo rustbasic route:list
```

## Contoh Controller
```rust
pub struct WelcomeController;

impl WelcomeController {
    pub async fn index(req: Request) -> impl IntoResponse {
        view(&req, "welcome.html", context! { 
            title => "Selamat Datang" 
        })
    }
}
```

---

# Views (Template)

RustBasic menggunakan engine **MiniJinja** yang sangat kuat. Template disimpan di `resources/views/`.

## Render View
Gunakan helper `view()`:
```rust
view(&req, "nama_file.html", context! { key => value })
```

## Template Inheritance
Di file `layout.html`:
```html
<html>
  <body>
    {% block content %}{% endblock %}
  </body>
</html>
```

Di file halaman (misal `home.html`):
```html
{% extends "layout.html" %}
{% block content %}
  <h1>Hello World</h1>
{% endblock %}

---

# Asset Management (Hidden Assets)

RustBasic memiliki sistem unik di mana asset inti (CSS & JS) disembunyikan dari folder publik dan ditanam langsung ke dalam binary aplikasi.

## Keuntungan
- **Keamanan**: User tidak bisa mendownload file `.css` atau `.js` secara langsung via URL.
- **Performa**: Asset diload langsung dari memori (RAM), nol disk I/O.
- **Standalone**: Aplikasi tidak bergantung pada file eksternal di folder publik untuk asset inti.

## Cara Penggunaan
Asset ini dikelola melalui komponen `assets.html`.

Di layout utama (`app.html`):
```html
{% from "components/assets.html" import styles, htmx %}
{{ styles() }}
{{ htmx() }}
```

File asli disimpan di:
- `resources/css/style.css`
- `resources/js/htmx.min.js`
```
