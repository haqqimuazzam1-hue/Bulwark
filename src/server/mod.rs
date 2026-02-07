use crate::request::context::RequestContext;
use crate::security::decision::DecisionEngine;
use crate::{BulwarkError, BulwarkResult, Decision};

/// Server
///
/// Server adalah eksekutor akhir.
/// Tidak punya logika security.
/// Hanya menjalankan hasil decision.
pub struct Server {
    decision_engine: DecisionEngine,
}

impl Server {
    pub fn new(decision_engine: DecisionEngine) -> Self {
        Self { decision_engine }
    }

    pub fn handle(&self, ctx: &RequestContext) -> BulwarkResult<()> {
        match self.decision_engine.decide(ctx) {
            Ok(Decision::Allow) => Ok(()),

            Ok(Decision::Log) => Ok(()),

            // Secara desain seharusnya tidak terjadi,
            // tapi WAJIB ditangani agar match exhaustif
            Ok(Decision::Block) => Err(BulwarkError::blocked("request blocked by decision")),

            Err(BulwarkError::Blocked { .. }) => {
                Err(BulwarkError::blocked("request blocked by bulwark"))
            }

            Err(err) => Err(err),
        }
    }
}
