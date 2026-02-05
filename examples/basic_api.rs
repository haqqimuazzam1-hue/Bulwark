use bulwark::request::context::{Method, RequestContext};
use bulwark::request::normalize::Normalizer;
use bulwark::security::decision::DecisionEngine;
use bulwark::security::inspector::BasicInspector;
use bulwark::Decision;
use bulwark::security::inspector_header_size::HeaderSizeInspector;
use bulwark::security::inspector_method::MethodInspector;
use bulwark::request::context::Method;
use bulwark::security::inspector_user_agent::UserAgentInspector;

fn main() {
    // 1️⃣ Buat request mentah (simulasi request masuk)
    let mut ctx = RequestContext::new(Method::POST, "///login//");

    ctx.insert_header(" User-Agent ", " curl/8.0 ");
    ctx.insert_query(" user ", " admin ");
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
    engine.add(BasicInspector);
    engine.add(HeaderSizeInspector::new(1024));
    engine.add(MethodInspector::new(vec![
    Method::GET,
    Method::POST,
]));
    engine.add(UserAgentInspector::new(vec![
    "sqlmap",
    "nmap",
    "nikto",
    "masscan",
    "curl",
    "python-request",
    "go-http-client",
]));


    // 4️⃣ Run security decision
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
                // Sebenarnya tidak akan ke sini
                println!("❌ BLOCK");
            }
        },
        Err(err) => {
            println!("❌ BLOCKED: {:?}", err);
        }
    }
}