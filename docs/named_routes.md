# 🏷️ Panduan Rute Bernama (Named Routes)

## 📝 Kata Pengantar
Selamat datang di panduan **Rute Bernama (Named Routes)** pada framework **RustBasic**. Dokumentasi ini dirancang khusus untuk memandu Anda memahami cara mendaftarkan nama unik pada rute backend Rust, serta bagaimana menghubungkannya secara dinamis dengan frontend React.js SPA (Single Page Application) menggunakan Inertia.js.

Konsep ini terinspirasi dari framework modern (menggunakan helper `route()`), yang menghindarkan Anda dari hardcoding URL secara manual pada frontend dan backend.

---

## 🛠️ Cara Kerja & Arsitektur

Saat aplikasi berjalan:
1. **Pendaftaran (Backend)**: Seluruh rute yang didefinisikan menggunakan metode `.name("nama.rute")` akan dikumpulkan ke dalam registry global statis (`NAMED_ROUTES`).
2. **Pengiriman Props (Inertia)**: Helper `inertia` di backend secara otomatis menyuntikkan daftar rute bernama beserta konfigurasi `APP_URL` ke dalam shared props Inertia (`props.routes` dan `props.app_url`).
3. **Penyajian Dinamis (Frontend)**: Hook `useRoute` di React membaca shared props tersebut untuk memetakan nama rute ke URL absolut, lengkap dengan dukungan penggantian parameter URL maupun query parameters otomatis.

---

## 🚀 Implementasi di Backend (`src/routes/web.rs`)

Anda cukup memanggil `.name("nama")` setelah mendefinisikan rute dengan `.route()` pada pembangun Router.

```rust
use rustbasic_core::{Router, get, AppState};
use crate::app::http::controllers::welcome_controller;

pub fn router() -> Router<AppState> {
    Router::new()
        // Rute statis dasar
        .route("/", get(welcome_controller::index)).name("home")
        .route("/about", get(welcome_controller::about)).name("about")
        
        // Rute dengan parameter dinamis tunggal
        .route("/test/:id", get(welcome_controller::test_param)).name("test.param")
        
        // Rute dengan multi parameter dinamis (> 3)
        .route("/test/multi/:p1/:p2/:p3/:p4", get(welcome_controller::test_multi_param)).name("test.multi")
}
```

---

## 💻 Implementasi di Frontend (React.js SPA)

### A. Menggunakan Hook `useRoute`
Impor hook `useRoute` dari berkas `src/resources/js/route.ts` dan panggil di dalam komponen React Anda.

```tsx
import { Link } from '@inertiajs/react';
import { useRoute } from '../route';

export default function MyComponent() {
  const route = useRoute();

  return (
    <div>
      {/* Rute tanpa parameter */}
      <Link href={route('home')}>Beranda</Link>
      <Link href={route('about')}>Tentang Kami</Link>
    </div>
  );
}
```

### B. Menangani Parameter URL
Metode `route` menerima argumen kedua berupa objek berisi parameter-parameter URL yang didefinisikan pada backend.

#### 1. Single Parameter
Jika rute backend didefinisikan sebagai `/test/:id` (nama: `test.param`):
```tsx
const url = route('test.param', { id: '99' });
// Menghasilkan: http://localhost:4000/test/99 (sesuai konfigurasi APP_URL di .env)
```

#### 2. Multi Parameter
Jika rute backend didefinisikan sebagai `/test/multi/:p1/:p2/:p3/:p4` (nama: `test.multi`):
```tsx
const url = route('test.multi', { 
  p1: 'apple', 
  p2: 'banana', 
  p3: 'orange', 
  p4: 'grape' 
});
// Menghasilkan: http://localhost:4000/test/multi/apple/banana/orange/grape
```

#### 3. Otomatis Menangani Query Parameters (Fallback)
Apabila Anda melewatkan key parameter tambahan yang tidak terdaftar di dalam pola URL backend, helper `useRoute` otomatis mengonversinya menjadi query string (`?key=value`):
```tsx
const url = route('test.param', { 
  id: '99', 
  search: 'rustbasic', 
  filter: 'active' 
});
// Menghasilkan: http://localhost:4000/test/99?search=rustbasic&filter=active
```

---

## ⚙️ Detail Teknis Penyebaran Data

Data rute dikirimkan sebagai properti global oleh helper `inertia()` secara otomatis setiap kali method `inertia()` dieksekusi:

```rust
let named_routes = rustbasic_core::router::get_named_routes();
map.insert("routes".to_string(), json!(named_routes));

let cfg = rustbasic_core::Config::load();
map.insert("app_url".to_string(), json!(cfg.app_url));
```

Dan di frontend React, data tersebut diurai secara asinkron lewat helper hook:

```typescript
// src/resources/js/route.ts
import { usePage } from '@inertiajs/react';

export function useRoute() {
  const { props } = usePage<any>();
  const routes = (props.routes || {}) as Record<string, string>;
  const appUrl = (props.app_url || '').replace(/\/$/, '');

  return (name: string, params?: Record<string, string | number>) => {
    let path = routes[name];
    if (!path) {
      console.warn(`Route named "${name}" not found.`);
      return name;
    }

    if (params) {
      const remainingParams = { ...params };
      Object.entries(params).forEach(([key, val]) => {
        const placeholder1 = `:${key}`;
        const placeholder2 = `{${key}}`;
        let replaced = false;

        if (path.includes(placeholder1)) {
          path = path.replace(placeholder1, String(val));
          replaced = true;
        }
        if (path.includes(placeholder2)) {
          path = path.replace(placeholder2, String(val));
          replaced = true;
        }

        if (replaced) {
          delete remainingParams[key];
        }
      });

      const queryKeys = Object.keys(remainingParams);
      if (queryKeys.length > 0) {
        const queryString = queryKeys
          .map(k => `${encodeURIComponent(k)}=${encodeURIComponent(String(remainingParams[k]))}`)
          .join('&');
        path += (path.includes('?') ? '&' : '?') + queryString;
      }
    }

    const cleanPath = path.startsWith('/') ? path : `/${path}`;
    return `${appUrl}${cleanPath}`;
  };
}
```

---

## 🏁 Kesimpulan
Dengan menggunakan sistem **Rute Bernama (Named Routes)**, perubahan path di backend Rust (misalnya mengubah `/about` menjadi `/tentang-kami`) tidak akan mematahkan link di frontend React Anda, karena frontend akan secara dinamis mengikuti perubahan path tersebut berdasarkan nama rute yang tetap konsisten.
