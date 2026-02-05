use crate::request::context::RequestContext;
use crate::{Decision, BulwarkError, BulwarkResult};
use super::inspector::Inspector;

/// Inspector untuk membatasi total ukuran header.
///
/// Tujuan:
/// - cegah header abuse
/// - kurangi attack surface
/// - ringan & deterministic
pub struct HeaderSizeInspector {
    max_bytes: usize,
}

impl HeaderSizeInspector {
    /// Buat inspector dengan batas ukuran header (dalam bytes).
    pub fn new(max_bytes: usize) -> Self {
        Self { max_bytes }
    }
}

impl Inspector for HeaderSizeInspector {
    fn inspect(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        let mut total_size = 0usize;

        for (k, v) in &ctx.headers {
            total_size += k.len();
            total_size += v.len();
        }

        if total_size > self.max_bytes {
            return Err(BulwarkError::blocked(
                "request headers too large",
            ));
        }

        Ok(Decision::Allow)
    }
}