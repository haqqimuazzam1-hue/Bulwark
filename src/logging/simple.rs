use crate::request::context::RequestContext;
use crate::Decision;

pub struct SimpleLogger;

impl SimpleLogger {
    pub fn log(ctx: &RequestContext, decision: &Decision, reason: &str) {
        let method = &ctx.method;
        let path = &ctx.path;

        match decision {
            Decision::Allow => {
                println!("[ALLOW] {:?} {} | {}", method, path, reason);
            }
            Decision::Log => {
                println!("[LOG] {:?} {} | {}", method, path, reason);
            }
            Decision::Block => {
                println!("[BLOCK] {:?} {} | {}", method, path, reason);
            }
        }
    }
}
