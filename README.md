BULWARK

Bulwark adalah framework security core berbasis Rust yang dirancang untuk membantu security developer membangun lapis pertahanan (WAF / security gateway) yang ringan, modular, dan deterministic.

Bulwark fokus pada security decision pipeline, bukan web framework. Ia bisa dipakai sebagai:
• core WAF
• security engine
• request inspection layer
• fondasi middleware (Axum / Actix / custom server)

FITUR UTAMA (v0.2)

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

Arsitektur:
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

Struktur Project:
src/
├── request/          # Request context & normalization
├── security/         # Inspector & decision engine
├── logging/          # Logging system
├── server/           # Request executor
├── lib.rs            # Public API
└── examples/         # Contoh penggunaan

Contoh Penggunaan:
use bulwark::request::context::RequestContext;
use bulwark::security::decision::DecisionEngine;
use bulwark::security::inspector_method::InspectorMethod;
use bulwark::security::inspector_header_size::InspectorHeaderSize;
use bulwark::security::inspector_user_agent::InspectorUserAgent;
use bulwark::Decision;

let mut ctx = RequestContext::new("POST", "/login");
ctx.insert_header("User-Agent", "curl/8.0");

let mut engine = DecisionEngine::new();

engine.add(InspectorMethod::new(vec!["GET", "POST"]));
engine.add(InspectorHeaderSize::new(64, 256));
engine.add(InspectorUserAgent::new(
    64,
    vec!["sqlmap", "nmap", "nikto"],
));

match engine.decide(&ctx) {
    Ok(Decision::Allow) => println!("ALLOW"),
    Ok(Decision::Log) => println!("LOG"),
    Err(_) => println!("BLOCK"),
}

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