use crate::request::context::RequestContext;
use crate::{Decision, BulwarkError, BulwarkResult};
use super::inspector::Inspector;

/// Inspector untuk mendeteksi User-Agent berbahaya.
///
/// Contoh yang sering dipakai attacker:
/// - sqlmap
/// - nmap
/// - nikto
/// - curl (opsional)
pub struct UserAgentInspector {
    blocked_agents: Vec<&'static str>,
}

impl UserAgentInspector {
    /// Buat UserAgentInspector dengan daftar substring UA yang diblokir
    pub fn new(blocked_agents: Vec<&'static str>) -> Self {
        Self { blocked_agents }
    }
}

impl Inspector for UserAgentInspector {
    fn inspect(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        let ua = match ctx.headers.get("user-agent") {
            Some(v) => v.to_lowercase(),
            None => return Ok(Decision::Allow), // tidak ada UA → tidak langsung block
        };

        for bad in &self.blocked_agents {
            if ua.contains(bad) {
                return Err(BulwarkError::blocked(
                    "blocked by user-agent inspector",
                ));
            }
        }

        Ok(Decision::Allow)
    }
}