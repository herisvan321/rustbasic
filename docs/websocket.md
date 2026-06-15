# WebSocket Event Broadcaster

RustBasic menyediakan sistem penyiaran event secara real-time (*WebSocket Event Broadcaster*) terpadu yang ringan dan berkinerja tinggi. Sistem ini memungkinkan backend Rust mengirimkan data secara instan (*server-to-client push*) ke halaman frontend ReactJS maupun klien eksternal secara asinkron.

---

## 🛠️ Konfigurasi (`.env`)

Fitur Broadcaster bersifat opsional dan dikonfigurasi melalui file `.env`. Untuk mengaktifkan fitur ini, tambahkan atau ubah baris berikut:

```env
# Aktifkan untuk menyalakan engine WebSocket di port aplikasi Anda pada route /ws
WEBSOCKET_ENABLED=true
```

Jika dinonaktifkan (`WEBSOCKET_ENABLED=false`), route `/ws` akan mengembalikan respons `404 Not Found` dan tidak ada koneksi WebSocket yang diterima.

---

## 📡 Penggunaan di Sisi Backend (Rust)

Untuk menyiarkan data atau pesan dari bagian manapun di backend Rust (seperti dari Controller atau Service), Anda dapat menggunakan helper statis `Broadcaster`:

```rust
use rustbasic_core::Broadcaster;
use rustbasic_core::serde_json::json;

// Menyiarkan pesan dengan payload JSON ke channel "demo-channel" dengan nama event "test-event"
Broadcaster::to("demo-channel")
    .emit("test-event", json!({
        "message": "Halo dari backend RustBasic!",
        "status": "success"
    }))
    .await;
```

---

## 💻 Penggunaan di Sisi Frontend (ReactJS / TypeScript)

Kami menyediakan klien WebSocket bawaan yang siap pakai di dalam berkas `src/resources/js/reverb.ts` (menggunakan nama modul `ReverbClient` untuk client-side helper).

### Cara Berlangganan Event:

```typescript
import { ReverbClient } from './reverb';

// 1. Inisialisasi Klien (Secara otomatis akan mendeteksi protokol ws/wss dan host saat ini)
const reverb = new ReverbClient();

// 2. Berlangganan ke Channel Tertentu
const channel = reverb.subscribe("demo-channel");

// 3. Mendengarkan Event Tertentu
channel.listen("test-event", (data) => {
    console.log("Pesan diterima:", data.message);
    
    // Contoh memicu notifikasi visual ke user
    alert(`Notifikasi Baru: ${data.message}`);
});
```

---

## 🔌 Integrasi API & Klien Eksternal (Decoupled)

Sistem WebSocket Broadcaster ini dirancang secara terpisah (*decoupled*). Klien apa pun yang mendukung protokol WebSocket standar (Aplikasi Mobile Native, Flutter, external script, dll.) dapat terhubung langsung ke server.

### 1. Endpoint Koneksi
* **URL:** `ws://<host>:<port>/ws` atau `wss://<host>:<port>/ws`

### 2. Protokol Komunikasi (JSON Frames)

#### A. Mendaftarkan Langganan (Subscribe)
Kirim payload berikut setelah koneksi WebSocket terbuka untuk mulai mendengarkan event di channel tertentu:
```json
{
  "action": "subscribe",
  "channel": "nama-channel-anda"
}
```

#### B. Membatalkan Langganan (Unsubscribe)
Kirim payload berikut untuk berhenti mendengarkan pesan dari channel tertentu:
```json
{
  "action": "unsubscribe",
  "channel": "nama-channel-anda"
}
```

#### C. Struktur Event yang Diterima Klien
Klien akan menerima pesan teks JSON dari server dengan format berikut setiap kali event dipicu di backend:
```json
{
  "event": "nama-event",
  "channel": "nama-channel",
  "data": {
    "key": "value"
  }
}
```

#### D. Mengirim Pesan dari Klien ke Klien Lain (2-Arah / Whisper)
Klien dapat menyebarkan pesan langsung via koneksi WebSocket (2-arah). Server akan otomatis mendistribusikan pesan tersebut ke seluruh pelanggan channel tersebut (kecuali si pengirim pesan itu sendiri).

Kirim payload teks JSON berikut lewat koneksi WebSocket Anda:
```json
{
  "action": "broadcast",
  "channel": "nama-channel-anda",
  "event": "nama-event",
  "data": {
    "message": "Halo dari browser client!"
  }
}
```

Atau dengan memanfaatkan helper client-side JavaScript secara terintegrasi:
```typescript
channel.broadcast("test-event", { message: "Halo dari browser client!" });
```
