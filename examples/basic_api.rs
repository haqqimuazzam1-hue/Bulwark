use bulwark::request::context::Methode;
use bulwark::request::normalize::Normalizer;
use bulwark::security::decision::DecisionEngine;
use bulwark::security::inspector_method::InspectorMethod;
use bulwark::security::inspector_header_size::InspectorHeaderSize;
use bulwark::security::inspector_user_agent::InspectorUserAgent;
use bulwark::Decision;

fn main() {
    // 1️⃣ Simulasi request masuk
    let mut ctx = RequestContext::new("POST", "///login//");

    ctx.insert_header("User-Agent", "curl/8.0");
    ctx.insert_header("X-Test", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    ctx.insert_query("user", "admin");
    ctx.set_body(b"username=admin&password=123".to_vec());
    ctx.set_client_ip("127.0.0.1");

    println!("== RAW REQUEST ==");
    println!("{:#?}", ctx);

    // 2️⃣ Normalize request
    Normalizer::normalize(&mut ctx);

    println!("\n== NORMALIZED REQUEST ==");
    println!("{:#?}", ctx);

    // 3️⃣ Setup decision engine
    let mut engine = DecisionEngine::new();

    engine.add(InspectorMethod::new(vec![
    Method::GET,
    Method::POST,
]));

    engine.add(InspectorHeaderSize::new(
        64,    // soft limit (log)
        256,   // hard limit (block)
    ));

    engine.add(InspectorUserAgent::new(
        64, // max UA length
        vec![
            "sqlmap",
            "nmap",
            "nikto",
            "masscan",
            "python",
            "curl",
        ],
    ));

    // 4️⃣ Jalankan decision
    println!("\n== SECURITY DECISION ==");

    match engine.decide(&ctx) {
        Ok(decision) => match decision {
            Decision::Allow => {
                println!("✅ ALLOW: request accepted");
            }
            Decision::Log => {
                println!("⚠️ LOG: request allowed but logged");
            }
            Decision::Block => {
                // Secara desain, Block biasanya lewat Err
                println!("❌ BLOCK");
            }
        },
        Err(err) => {
            println!("❌ BLOCKED: {:?}", err);
        }
    }
}