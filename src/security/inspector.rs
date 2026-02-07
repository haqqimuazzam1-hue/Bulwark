use crate::request::context::RequestContext;
use super::decision::FindingSeverity;

/// InspectorFinding
///
/// Hasil temuan dari sebuah inspector.
/// Inspector TIDAK BOLEH mengambil keputusan,
/// hanya melaporkan apa yang dia temukan.
#[derive(Debug, Clone)]
pub struct InspectorFinding {
    /// Nama inspector (untuk logging & debugging)
    pub inspector: &'static str,

    /// Tingkat keparahan temuan
    pub severity: FindingSeverity,

    /// Alasan / penjelasan singkat
    pub reason: String,
}

impl InspectorFinding {
    pub fn new(
        inspector: &'static str,
        severity: FindingSeverity,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            inspector,
            severity,
            reason: reason.into(),
        }
    }
}

/// Inspector trait
///
/// Semua inspector HARUS mengimplementasikan trait ini.
///
/// Kontrak penting:
/// - Ok(None)        → tidak ada masalah
/// - Ok(Some(finding)) → ada temuan
/// - Err(...)        → error fatal (pipeline berhenti)
pub trait Inspector: Send + Sync {
    fn inspect(
        &self,
        ctx: &RequestContext,
    ) -> Result<Option<InspectorFinding>, crate::BulwarkError>;
}