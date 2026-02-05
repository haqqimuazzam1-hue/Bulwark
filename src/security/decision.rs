use crate::request::context::RequestContext;
use crate::{Decision, BulwarkError, BulwarkResult};
use crate::logging::simple::SimpleLogger;
use super::inspector::Inspector;

/// DecisionEngine
///
/// Menjalankan semua inspector secara berurutan
/// dan menghasilkan satu keputusan final.
///
/// Prioritas:
/// BLOCK > LOG > ALLOW
pub struct DecisionEngine {
    inspectors: Vec<Box<dyn Inspector>>,
}

impl DecisionEngine {
    /// Membuat decision engine kosong.
    pub fn new() -> Self {
        Self {
            inspectors: Vec::new(),
        }
    }

    /// Menambahkan inspector ke pipeline.
    pub fn add<I>(&mut self, inspector: I)
    where
        I: Inspector + 'static,
    {
        self.inspectors.push(Box::new(inspector));
    }

    /// Menjalankan semua inspector dan menentukan keputusan akhir.
    pub fn decide(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        let mut final_decision = Decision::Allow;

        for inspector in &self.inspectors {
            match inspector.inspect(ctx) {
                Ok(decision) => match decision {
                    Decision::Allow => {}
                    Decision::Log => {
                        final_decision = Decision::Log;
                        SimpleLogger::log(ctx, &Decision::Log, "flagged by inspector");
                    }
                    Decision::Block => {
                        SimpleLogger::log(ctx, &Decision::Block, "blocked by inspector");
                        return Err(BulwarkError::blocked("blocked by security rule"));
                    }
                },
                Err(err) => {
                    SimpleLogger::log(ctx, &Decision::Block, "blocked by inspector error");
                    return Err(err);
                }
            }
        }

        Ok(final_decision)
    }
}