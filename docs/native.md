# 📱💻 Dokumentasi RustBasic Native (Mobile & Desktop Wrappers)

RustBasic Native memungkinkan Anda menjalankan aplikasi web full-stack RustBasic (React + Inertia SPA) sebagai aplikasi **Desktop Native (macOS/Windows/Linux)** dan **Mobile Native (Android/iOS)**.

Sistem ini menggunakan **Wry WebView** untuk Desktop dan **System WebView** untuk Mobile. Seluruh logika server dan database SQLite berjalan langsung secara lokal (*zero-latency local backend*) di dalam perangkat pengguna.

---

## 🛠️ 1. Prasyarat Sistem (Prerequisites)

Sebelum memulai pengembangan native, pastikan Anda telah memasang dependensi berikut di komputer Anda:

### Desktop & Mobile Common
*   **Rust & Cargo** (v1.75 atau lebih tinggi).
*   **Node.js & npm** (Untuk build frontend React).

### Khusus Android
*   **Android Studio**: Pasang versi terbaru untuk mendapatkan Android SDK.
*   **Android NDK (Native Development Kit)**: Pasang NDK melalui Android Studio (`Tools -> SDK Manager -> SDK Tools -> NDK (Side by side)`).
*   **Java JDK**: Disarankan menggunakan JBR (JetBrains Runtime) bawaan Android Studio atau JDK 17+.

---

## 📦 2. Cara Instalasi (Scaffolding)

Wrapper Native tidak dimasukkan secara default di Starter Kit agar proyek web Anda tetap bersih. Anda dapat menambahkannya kapan saja secara *opt-in*.

1.  Buka terminal di direktori utama proyek RustBasic Anda.
2.  Jalankan perintah penginstalan scaffolding native:
    ```bash
    rustbasic-native install
    ```
3.  Perintah ini secara otomatis akan:
    *   Membaca port (`APP_PORT`) dan kunci keamanan (`APP_KEY`) aktif dari `.env`.
    *   Membuat folder `native/android/` yang berisi kerangka proyek Android Studio (Kotlin).
    *   Membuat folder `native/desktop/` yang berisi kode pembungkus Wry Rust.
    *   Membuat berkas utilitas penunjang (JNI Bridge, bash runner script, dll.).
    *   Menambahkan target library `staticlib` dan `cdylib` di file `Cargo.toml` proyek utama Anda.

---

## 🚀 3. Penggunaan di Mode Pengembangan (Development)

Untuk mempermudah pengembangan dengan fitur **Hot Module Replacement (HMR)** dari Vite, ikuti langkah berikut:

### Langkah A: Jalankan Vite Dev Server
```bash
npm run dev
```

### Langkah B: Jalankan Aplikasi Wrapper

Buka terminal baru di direktori utama proyek Anda dan jalankan perintah target:

#### Untuk Desktop:
```bash
rustbasic serve --desktop
```
CLI akan mendeteksi apakah Vite sedang aktif di port `5173`. Jika aktif, window desktop akan terbuka dan memuat aset real-time langsung dari Vite dengan kemampuan auto-reload saat Anda mengubah komponen React.

#### Untuk Android:
Pastikan perangkat Android fisik (dengan USB Debugging aktif) atau Emulator Android sudah terhubung dan aktif di ADB, lalu jalankan:
```bash
rustbasic serve --android
```
CLI akan mengompilasi JNI library Rust, memasang APK debug ke perangkat, menyalakan server lokal di background ponsel, dan menampilkan log realtime server langsung ke terminal Anda (*tailing logcat*).

---

## 🔌 4. Native JavaScript Bridge

RustBasic menyediakan jembatan komunikasi antara halaman web React/TypeScript dengan API perangkat keras native ponsel via `window.MobileBridge`.

Secara default, fungsi yang tersedia meliputi:

```javascript
// 1. Meminta data GPS / Lokasi Perangkat
window.MobileBridge.getGPSLocation().then(data => {
    console.log("Latitude: " + data.latitude);
    console.log("Longitude: " + data.longitude);
});

// 2. Membaca Sensor Perangkat (Baterai, Proximity, dll.)
window.MobileBridge.getDeviceSensors().then(data => {
    console.log("Battery: " + data.battery + "%");
});

// 3. Menampilkan Notification Toast Native
window.MobileBridge.showToast("Halo dari React!");
```

### Cara Menambahkan Fungsi Kustom di Android (Kotlin):
1.  Buka berkas `native/android/app/src/main/java/com/rustbasic/mobile/MainActivity.kt`.
2.  Tambahkan fungsi baru beranotasi `@JavascriptInterface` di dalam kelas `MobileBridge`:
    ```kotlin
    class MobileBridge(private val activity: MainActivity) {
        @JavascriptInterface
        fun getDeviceBrand(): String {
            return android.Build.MANUFACTURER
        }
    }
    ```
3.  Di file JavaScript/React, Anda dapat langsung memanggilnya:
    ```javascript
    let brand = window.MobileBridgeNative.getDeviceBrand();
    ```

---

## 🚢 5. Tahap Kompilasi Rilis & Publish (Production Build)

Saat aplikasi siap dirilis ke pengguna umum atau diunggah ke app store, kompilasi aplikasi Anda menggunakan mode **Release (Production)**.

### A. Build Aplikasi Desktop
```bash
rustbasic build --desktop
```
**Proses otomatis:**
1.  Menjalankan `npm run build` untuk memaketkan dan mengoptimasi semua berkas React SPA ke folder `src/dist`.
2.  Mengompilasi wrapper desktop dalam mode release (`cargo build --release`). Seluruh aset web produksi di-embed langsung ke dalam berkas binary.
3.  Executable release Anda akan dihasilkan di: `native/desktop/target/release/rustbasic-native-desktop`.

---

### B. Build Aplikasi Android (APK & AAB)
```bash
rustbasic build --android
```
**Proses otomatis:**
1.  Menjalankan `npm run build` untuk memaketkan aset web produksi.
2.  Menjalankan kompilasi silang library Rust JNI untuk multi-arsitektur CPU mobile (`arm64`, `armv7`, `x86_64`) secara teroptimasi.
3.  Secara otomatis mengompilasi berkas **APK** (untuk pemasangan langsung) dan **AAB** (untuk diunggah ke Google Play Store).
4.  Hasil keluaran build:
    *   **Signed APK**: `native/android/app/build/outputs/apk/release/app-release.apk`
    *   **Signed AAB**: `native/android/app/build/outputs/bundle/release/app-release.aab`

---

## 🔑 C. Pengaturan Keystore Rilis Resmi (Play Store)

Secara default, CLI akan menghasilkan berkas keystore uji coba `release.keystore` dengan sandi `rustbasic` agar kompilasi rilis Anda tidak gagal dan menghasilkan APK siap guna.

Namun, untuk mengunggah aplikasi ke **Google Play Store**, Anda harus menandatangani aplikasi secara manual menggunakan sertifikat keystore milik Anda sendiri:

### 1. Buat Keystore Anda Sendiri
Jalankan perintah berikut di terminal Anda untuk menghasilkan berkas kunci keystore:
```bash
keytool -genkey -v -keystore my-production-key.keystore -alias my-key-alias -keyalg RSA -keysize 2048 -validity 10000
```
Simpan file `my-production-key.keystore` yang dihasilkan di tempat yang aman.

### 2. Konfigurasi di Gradle
Salin berkas `my-production-key.keystore` ke dalam direktori `native/android/app/`. 

Buka berkas [native/android/app/build.gradle](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/native/android/app/build.gradle) lalu sesuaikan blok `signingConfigs` menggunakan data Anda:

```groovy
android {
    ...
    signingConfigs {
        release {
            storeFile file("my-production-key.keystore")
            storePassword "password-keystore-anda"
            keyAlias "my-key-alias"
            keyPassword "password-key-anda"
        }
    }

    buildTypes {
        release {
            signingConfig signingConfigs.release
            minifyEnabled false
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
}
```

*Catatan: Begitu CLI mendeteksi adanya blok `signingConfigs` manual yang Anda buat di build.gradle, CLI secara otomatis akan mematikan auto-signing uji coba dan menyerahkan sepenuhnya pada konfigurasi manual Anda.*

---

## 💎 6. Fitur Premium & Kepatuhan WebView (Play Store Compliance)

Untuk memastikan kelancaran saat dipublikasikan ke Google Play Console dan mencegah penolakan (rejection) terkait WebView biasa, pembungkus Android RustBasic telah ditingkatkan dengan standar premium native:

### A. Pengaman Server Satu-Instance (Process-Wide Atomic Gate)
Untuk mencegah panic `AddrInUse` (Address already in use) saat orientasi layar ponsel berubah atau saat aplikasi dimuat ulang dalam proses yang sama:
* Rust native JNI menggunakan gerbang logika atomik `SERVER_RUNNING` (`AtomicBool`).
* Sistem mendeteksi secara langsung di memori tingkat proses OS dan mencegah instansiasi server duplikat, sehingga thread server lama tidak bertabrakan dengan thread server baru.

### B. Sinkronisasi Port & URL Dinamis (CSP Fix)
Untuk menghindari pemblokiran kebijakan keamanan browser/WebView (Content Security Policy) saat melakukan navigasi halaman atau pengiriman form (Inertia SPA):
* Nilai port dibaca secara dinamis dari berkas `.env` (`APP_PORT`) dan disuntikkan ke dalam `build.gradle` Android.
* File `Config::load()` di `rustbasic-core` akan menyelaraskan port pada properti `app_url` secara dinamis dan otomatis jika URL tersebut menunjuk ke localhost. Ini memastikan semua permintaan navigasi dan fetch internal (seperti ke rute `/about`) disalurkan secara aman dalam domain yang sama (`'self'`).

### C. Sinkronisasi Nama Aplikasi (.env Sync)
Judul aplikasi pada layar utama (home screen) ponsel Android dan logo pada splash screen pemuatan tersinkronisasi langsung dengan file `.env` proyek utama Anda:
* Variabel `APP_NAME` di `.env` dibaca oleh Gradle dan disuntikkan ke dalam string resource `@string/app_name`.
* `AndroidManifest.xml` (`android:label`) dan elemen teks `MainActivity.kt` memuat resource tersebut secara dinamis.

### D. Tampilan Antarmuka Native Premium (Programmatic Overlays)
WebView dibungkus dengan layout programatik modern yang memberikan visualisasi premium dan mulus:
1.  **Splash & Loading Spinner Overlay**: Tampilan pemuatan bertema gelap (dark mode) dengan spinner putar yang menutupi waktu bootstrap server Rust (sekitar 5.2 detik). Overlay ini akan memudar secara halus menggunakan animasi transisi (`animate().alpha(0f)`) setelah halaman siap.
2.  **Offline & Connection Error Overlay**: Tampilan kesalahan native bertema senada yang akan aktif secara otomatis jika koneksi server terputus atau gagal dijangkau, lengkap dengan tombol **"Coba Lagi"** untuk memuat ulang WebView secara non-blocking.
3.  **Dukungan File Chooser & Sensor**: Dilengkapi dengan `WebChromeClient` kustom untuk penanganan upload berkas HTML5 dari galeri/penyimpanan ponsel, serta persetujuan izin lokasi (geolocation) secara native.

