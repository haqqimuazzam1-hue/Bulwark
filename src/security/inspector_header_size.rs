use crate::request::context::RequestContext;
use crate::security::decision::FindingSeverity;
use crate::security::inspector::{Inspector, InspectorFinding};
use crate::BulwarkError;

/// InspectorHeaderSize
///
/// Mengecek total ukuran header request.
/// Fokus: mencegah abuse header yang terlalu besar.
pub struct InspectorHeaderSize {
    /// Batas aman (byte)
    soft_limit: usize,
    /// Batas keras (byte)
    hard_limit: usize,
}

impl InspectorHeaderSize {
    /// Membuat inspector dengan dua threshold:
    /// - soft_limit  -> Medium (log)
    /// - hard_limit  -> High (block)
    pub fn new(soft_limit: usize, hard_limit: usize) -> Self {
        Self {
            soft_limit,
            hard_limit,
        }
    }

    /// Hitung total ukuran header (nama + nilai)
    fn calculate_total_size(ctx: &RequestContext) -> usize {
        ctx.headers.iter().map(|(k, v)| k.len() + v.len()).sum()
    }
}

impl Inspector for InspectorHeaderSize {
    fn inspect(&self, ctx: &RequestContext) -> Result<Option<InspectorFinding>, BulwarkError> {
        let total_size = Self::calculate_total_size(ctx);

        // HARD LIMIT → High severity (Block)
        if total_size > self.hard_limit {
            return Ok(Some(InspectorFinding::new(
                "inspector_header_size",
                FindingSeverity::High,
                format!(
                    "header size {} bytes exceeds hard limit {}",
                    total_size, self.hard_limit
                ),
            )));
        }

        // SOFT LIMIT → Medium severity (Log)
        if total_size > self.soft_limit {
            return Ok(Some(InspectorFinding::new(
                "inspector_header_size",
                FindingSeverity::Medium,
                format!(
                    "header size {} bytes exceeds soft limit {}",
                    total_size, self.soft_limit
                ),
            )));
        }

        // Aman
        Ok(None)
    }
}
