use bulwark::request::context::{Method, RequestContext};

fn main() {
    // Simulasi request masuk
    let ctx = RequestContext::new(Method::POST, "/login");

    log_request(&ctx);

    if is_login(&ctx) {
        println!("Login request detected");
    }
}

fn log_request(ctx: &RequestContext) {
    println!("Incoming Request");
    println!("  Method : {:?}", ctx.method);
    println!("  Path   : {}", ctx.path);
}

fn is_login(ctx: &RequestContext) -> bool {
    ctx.method == Method::POST && ctx.path == "/login"
}
