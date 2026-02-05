use crate::request::context::RequestContext;
use crate::Decision;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SimpleLogger;

impl SimpleLogger {
    pub fn log(ctx: &RequestContext, _decision: &Decision, reason: &str) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!(
            "[{}] {:?} {} | ip={:?} | reason={}",
            ts,
            ctx.method,
            ctx.path,
            ctx.client_ip,
            reason
        );
    }
}