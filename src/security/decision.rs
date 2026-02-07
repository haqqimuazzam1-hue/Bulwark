use crate::request::context::RequestContext;
use crate::logging::simple::SimpleLogger;
use crate::{BulwarkError, BulwarkResult, Decision};
use super::inspector::{Inspector, InspectorFinding};

/// DecisionEngine
///
/// Satu-satunya komponen yang BERHAK
/// menentukan keputusan security.
///
/// Alur:
/// Request -> Inspectors -> Decision -> Logging -> Server
pub struct DecisionEngine {
    inspectors: Vec<Box<dyn Inspector>>,
}

impl DecisionEngine {
    /// Membuat decision engine kosong
    pub fn new() -> Self {
        Self {
            inspectors: Vec::new(),
        }
    }

    /// Menambahkan inspector ke pipeline
    pub fn add<I>(&mut self, inspector: I)
    where
        I: Inspector + 'static,
    {
        self.inspectors.push(Box::new(inspector));
    }

    /// Menjalankan seluruh inspector dan menghasilkan keputusan final
    ///
    /// Prioritas:
    /// BLOCK > LOG > ALLOW
    pub fn decide(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        let mut findings: Vec<InspectorFinding> = Vec::new();

        // 1. Jalankan semua inspector
        for inspector in &self.inspectors {
            match inspector.inspect(ctx) {
                Ok(Some(finding)) => findings.push(finding),
                Ok(None) => {}
                Err(err) => {
                    // Inspector error = hard fail
                    SimpleLogger::log(ctx, &Decision::Block, "inspector error");
                    return Err(err);
                }
            }
        }

        // 2. Tentukan keputusan berdasarkan findings
        let decision = Self::evaluate_findings(&findings);

        // 3. Logging terpusat
        for finding in &findings {
            SimpleLogger::log(ctx, &decision, &finding.reason);
        }

        // 4. Kembalikan keputusan
        match decision {
            Decision::Block => {
                Err(BulwarkError::blocked("request blocked by security decision"))
            }
            _ => Ok(decision),
        }
    }

    /// Mengevaluasi semua temuan inspector
    fn evaluate_findings(findings: &[InspectorFinding]) -> Decision {
        let mut decision = Decision::Allow;

        for finding in findings {
            match finding.severity {
                FindingSeverity::High => return Decision::Block,
                FindingSeverity::Medium => decision = Decision::Log,
                FindingSeverity::Low => {}
            }
        }

        decision
    }
}

/// Tingkat keparahan temuan inspector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Low,
    Medium,
    High,
}