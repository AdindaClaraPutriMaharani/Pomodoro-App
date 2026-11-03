# POMO — Pixel Pomodoro (Tauri v2)

Jam pomodoro pixel art tone pink, kecil dan ringan.

## Prasyarat
- Node.js
- Rust (`rustup`)
- Windows: Microsoft C++ Build Tools + WebView2 (sudah ada di Windows 11)

## Cara menjalankan
```bash
npm install
npx tauri icon app-icon.png   # sekali saja, membuat semua ikon app
npm run dev                   # jalankan app
npm run build                 # bikin installer (.exe / .msi) di src-tauri/target/release/bundle
```

## Struktur
- `src/index.html`        : seluruh UI + animasi + logika timer
- `src-tauri/`            : sisi Rust (jendela, notifikasi, izin)
- `src-tauri/tauri.conf.json` : ukuran jendela (280x410), tanpa title bar bawaan
- `app-icon.png`          : ikon sumber 1024px

## Catatan
- Tombol pin di title bar = *always on top*. Minimize dan close sudah berfungsi.
- Tombol TES = durasi singkat (15 detik fokus, 8 detik istirahat) untuk cek animasi.
- Font Silkscreen dimuat dari Google Fonts (butuh internet). Untuk offline, unduh
  font-nya ke `src/` dan ganti `<link>` dengan `@font-face`.
- Kalau tinggi jendela terasa kurang/lebih, ubah `height` di `tauri.conf.json`.
