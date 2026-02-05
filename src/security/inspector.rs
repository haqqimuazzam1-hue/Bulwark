use crate::request::context::RequestContext;
use crate::{Decision, BulwarkError, BulwarkResult};

/// Inspector trait.
///
/// Inspector bertugas:
/// - menganalisis request
/// - mengembalikan Decision
///
/// Inspector TIDAK:
/// - kirim response
/// - terminate server
pub trait Inspector {
    fn inspect(&self, ctx: &RequestContext) -> BulwarkResult<Decision>;
}

/// Built-in inspector: basic sanity checks.
///
/// Ini inspector pertama Bulwark.
/// Kecil, jelas, dan realistis.
pub struct BasicInspector;

impl Inspector for BasicInspector {
    fn inspect(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        // 1. Block empty path
        if ctx.path.trim().is_empty() {
            return Err(BulwarkError::blocked("empty request path"));
        }

        // 2. Block missing User-Agent
        if ctx.header("user-agent").is_none() {
            return Err(BulwarkError::blocked("missing user-agent header"));
        }

        // 3. Block suspicious path traversal
        if ctx.path.contains("..") {
            return Err(BulwarkError::blocked("path traversal detected"));
        }

        // 4. Log requests with empty body on POST
        if ctx.method == crate::request::context::Method::POST && !ctx.has_body() {
            return Ok(Decision::Log);
        }

        Ok(Decision::Allow)
    }
}