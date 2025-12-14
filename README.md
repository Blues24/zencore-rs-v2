# Zencore

> Modular Rust-based toolkit for archiving, encryption, theming, and automation —  
> built with a strict, scalable multi-crate architecture.

<p align="center">
  <img src="./assets/icon.png" width="500"/>
</p>

<p align="center">
  <b>Archive • Encrypt • Theme • Automate • Extend</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-alpha-orange" />
  <img src="https://img.shields.io/badge/language-rust-informational" />
  <img src="https://img.shields.io/badge/License-AGPL_v3-blue.svg" />
</p>

---

## 🚀 Overview

**Zencore** adalah proyek Rust multi-crate yang menyediakan:

- **Core engine** untuk archiving, compression, encryption, theming, dan filesystem utilities  
- **Dua antarmuka**: CLI dan TUI  
- **Arsitektur modular** untuk menjaga codebase tetap bersih dan scalable  
- **Fokus pada kinerja, keamanan, dan fleksibilitas**

Tujuan akhirnya adalah menjadikan Zencore sebagai **toolset lintas-platform** untuk penggunaan pribadi dan profesional—baik sebagai aplikasi standalone maupun sebagai library.

---

# 📦 Workspace Structure

Zencore menggunakan struktur workspace Rust yang terpisah ketat per domain.

```

zencore/
├─ zencore_core/        # High-level orchestrator; API terpadu untuk CLI/TUI
├─ zencore_archive/     # Packing, unpacking, tar.gz, tar.zst, detection
├─ zencore_crypto/      # Encryption, decryption, hashing
├─ zencore_config
├─ zencore_theme/       # Theme engine (scriptable)
├─ zencore_utils/       # Logging, path tools, state, config, banner utils
├─ zencore_cli/         # Command-line interface
└─ zencore_tui/         # Terminal UI (ratatui/crossterm)

````

Setiap crate berdiri sendiri, bebas dari god-module antipattern.

---

# ✨ Features

### Core Features
| Fitur | Status | Deskripsi |
|-------|--------|-----------|
| Archiving | 🟡 In Progress | Packing/unpacking + compression support |
| Encryption | 🟡 In Progress | Encrypt/decrypt file & archive |
| Hashing | 🟡 Planned | SHA-256 / BLAKE3 untuk verifikasi |
| Theme Engine | 🟡 In Progress | Scriptable theme system |
| Config System | 🟡 In Progress | Auto-generate config + runtime overrides |
| CLI | 🟡 In Progress | Command-based workflow |
| TUI | 🟡 In Progress | Navigasi interaktif |
| Utils | 🟡 In Progress | Logger, path utils, fuzzer, state, banner |

---

# 🧱 Architecture Principles

Zencore mengikuti prinsip berikut:

### 1. **Separation of Concerns**
Setiap crate berfokus pada satu domain.  
Contoh: crypto tidak boleh tahu tentang archive.

### 2. **Low Coupling, High Cohesion**
Crate harus:

- Cohesive di dalam domainnya  
- Tidak bergantung silang tanpa alasan kuat  
- Tidak memiliki import “shortcut magic” antar domain  

### 3. **Async-first**
Karena beberapa fitur (theming, file operations, future plugins) membutuhkan async.

### 4. **Predictable Dependencies**
Tidak ada crate “serbaguna”.  
`zencore-utils` pun hanya berisi hal-hal fundamental.

### 5. **Extensible for packaging**
Arsitektur dari awal dirancang untuk:

- Windows build  
- Linux distro packaging  
- macOS bundling  
- Static binary / portable mode  

---

# ⚙️ Installation (Development)

Clone repo dan build seluruh workspace:

```bash
git clone https://github.com/your/Zencore.git
cd Zencore
cargo build --workspace --release
````

Menjalankan CLI:

```bash
cargo run -p zencore-cli -- <command>
```

Menjalankan TUI:

```bash
cargo run -p zencore-tui
```

---

# 🧪 Testing

Menjalankan test seluruh crate:

```bash
cargo test --workspace
```

Menjalankan test per crate:

```bash
cargo test -p zencore-archive
```

---

# 🔌 Crate Responsibilities (Detail)

### **zencore-core**

* API terpadu untuk CLI & TUI
* Mengikat archive, crypto, theme, config, dan utils
* Menyediakan high-level action seperti:

  * `core.pack_dir()`
  * `core.encrypt_file()`
  * `core.apply_theme()`

### **zencore-archive**

* Deteksi format (tar, tar.gz, tar.zst, zip planned)
* Templating nama archive
* Packing/unpacking
* Fuzzer untuk target directory

### **zencore-crypto**

* Encrypt/decrypt (age)
* Key handling
* File-based operations

### **zencore-theme**

* Theme loader
* Scripting engine (Lua, planned)
* Resource path resolver

### **zencore-utils**

* Logging
* Banner printer
* Path operations
* Config loader & generator
* State tracking

---

# 🖥️ CLI Quickstart

Contoh perintah (mock):

```bash
zencore pack ./project --out ./backup.zenc
zencore encrypt ./backup.zenc
zencore theme apply solarized
```

---

# 🖼️ TUI Preview (planned)

* File tree navigation
* Archive creator
* Status monitor
* Theme browser

---

# 📘 Configuration

Zencore otomatis membuat file konfigurasi ketika pertama kali dijalankan.

Lokasi default:

* **Linux**: `~/.config/zencore/config.toml`
* **Windows**: `%APPDATA%/Zencore/config.toml`
* **macOS**: `~/Library/Application Support/Zencore/config.toml`

---

# 🔮 Roadmap

| Tahap   | Target                              |
| ------- | ----------------------------------- |
| **0.1** | Core crates selesai + CLI basic     |
| **0.2** | Theme engine + TUI minimal          |
| **0.3** | Packaging untuk Windows/Linux/macOS |
| **0.4** | Plugin architecture                 |
| **0.5** | Online theme store / update checker |

---

# 🤝 Contributing

1. Fork repo
2. Gunakan branch per fitur
3. Tulis unit test
4. Pastikan linter & formatting lulus
5. Submit PR dengan penjelasan ringkas

---

# 📜 License

AGPL-V3 License.
Lihat file `LICENSE`.

---

# 🧩 Why Zencore Exists

Karena tool modern seharusnya modular, tidak monolit, tidak meledak oleh technical debt, dan mudah dibawa ke platform manapun.
Zencore ingin menjadi contoh pendekatan *clean architecture* dalam proyek Rust multi-crate yang terstruktur rapih.

---

<p align="center"><b>Built with Rust. Designed for longevity.</b></p>

---
