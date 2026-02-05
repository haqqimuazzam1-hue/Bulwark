BULWARK

Bulwark adalah framework security core berbasis Rust yang dirancang untuk membantu security developer membangun lapis pertahanan (WAF / security gateway) yang ringan, modular, dan deterministic.

Bulwark fokus pada security decision pipeline, bukan web framework. Ia bisa dipakai sebagai:
• core WAF
• security engine
• request inspection layer
• fondasi middleware Axum / Actix (v0.2)

FITUR UTAMA (v0.1)

• Request Normalization
Mencegah bypass dengan path, header, dan query normalization.

• Inspector-based Architecture
Setiap rule adalah Inspector terpisah dan modular.

• Decision Engine
Menghasilkan keputusan final: Allow, Log, atau Block.

• Security Logging
Logging ringan untuk request mencurigakan dan diblokir.

• Ringan & HP-friendly
Bisa dikembangkan dan dijalankan di HP (Ram kecil).

Arsitektur:
Request
  ↓
Normalize
  ↓
Inspector Pipeline
  ├─ BasicInspector
  ├─ HeaderSizeInspector
  ├─ MethodInspector
  ├─ UserAgentInspector
  ↓
Decision Engine
  ↓
Allow / Log / Block

Struktur Project:
src/
├── request/          # Request context & normalization
├── security/         # Inspector & decision engine
├── logging/          # Logging system
├── lib.rs            # Public API
└── examples/         # Contoh penggunaan

Contoh Penggunaan:
let mut engine = DecisionEngine::new();
engine.add(BasicInspector);
engine.add(HeaderSizeInspector::new(1024));
engine.add(MethodInspector::new(vec![Method::GET, Method::POST]));
engine.add(UserAgentInspector::new(vec!["sqlmap", "nmap"]));

match engine.decide(&ctx) {
    Ok(Decision::Allow) => println!("ALLOW"),
    Ok(Decision::Log) => println!("LOG"),
    Err(_) => println!("BLOCK"),
}

Roadmap:
v0.1 (sekarang)
• Core security engine
• Inspector system
• Logging

v0.2 (planned)
• Config-based engine
• Response helper
• Axum / Actix integration
• Rate limiting

Filosofi:
"Security should be boring, predictable, amd hard to bypass."

Bulwark tidak mengejar magic, tapi kejelasan dan kontrol penuh bagi security developer.

Status:
Experimental - API bisa berubah

License:
MIT License

Dibangun dengan Rust dan mindset security-first.