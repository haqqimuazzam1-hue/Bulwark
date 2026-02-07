![Rust](https://img.shields.io/badge/language-Rust-orange)
![Version](https://img.shields.io/badge/version-v0.2.1-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## BULWARK

Bulwark adalah framework security core berbasis Rust yang dirancang untuk membantu security developer membangun lapis pertahanan (WAF / security gateway) yang ringan, modular, dan deterministic.

Bulwark fokus pada security decision pipeline, bukan web framework. Ia bisa dipakai sebagai:
• core WAF
• security engine
• request inspection layer
• fondasi middleware (Axum / Actix / custom server)

_________________________________________

## FITUR UTAMA (v0.2)

• Request Normalization
Mencegah Bypass dengan normalisasi
• path
• header
• query

• Inspector-based Architecture
Setiap rule security adalah Inspector terpisah:
• modular
• mudah ditambah
• mudah diuji
Inspector tidak mengambil keputusan - hanya mendeteksi.

• Centralized Decision Engine
Semua hasil Inspector dikumpulkan dan diproses oleh Decision Engine.
Keputusan Akhir:
• Allow
• Log
• Block
Decision hanya satu pintu, konsisten dan dapat diaudit.

• Security Logging
Logging ringan berbasis decision:
tidak noisy
tidak liar
mengikuti hasil final (Allow / Log / Block)

• Ringan & HP-friendly
• tanpa magic
• tanpa async berlebihan
• hasil predictable
• cocok untuk resource terbatas

_________________________________________

## Arsitektur:
```teks
Request
  ↓
Normalize
  ↓
Inspector Pipeline
  ├─ Method Inspector
  ├─ Header Size Inspector
  ├─ User-Agent Inspector
  ↓
Decision Engine
  ↓
Logging
  ↓
Allow / Log / Block
```
_________________________________________

## Project Structure
```text
Bulwark/
├── src/
│   ├── server/          # Core server logic
│   ├── security/        # Decision engine & inspectors
│   ├── logging/         # Logging implementations
│   ├── request/         # Request context & method definitions
│   └── lib.rs           # Library entry point
├── examples/
│   └── basic_api.rs     # Basic usage example
├── Cargo.toml           # Project configuration
├── Cargo.lock           # Dependency lockfile
├── README.md            # Project documentation
└── CHANGELOG.md         # Release notes
```
_________________________________________

## Installation:
```bash
1. git clone https://github.com/haqqimuazzam1-hue/Bulwark.git
2. cd Bulwark
3. cargo build
```
_________________________________________

Roadmap:
v0.2.0 (current)
✅ Decision end-to-end
✅ Inspector refactor (finding-based)
✅ Centralized logging
✅ Server executor
✅ Example end-to-end

v0.3.0 (planned)
Config-based engine
Inspector enable / disable
Axum / Actix integration
Rate limiting
Structured logging (JSON)

Filosofi:
"Security should be boring, predictable, and hard to bypass."

Bulwark tidak mengejar magic atau AI hype.
Bulwark mengejar kejelasan, kontrol, dan determinisme.

Status:
Experimental - API bisa berubah

License:
MIT License

Dibangun dengan Rust dan mindset security-first.
