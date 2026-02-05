use crate::request::context::{RequestContext, Method};
use crate::{Decision, BulwarkError, BulwarkResult};
use super::inspector::Inspector;

/// Inspector untuk membatasi HTTP method yang diizinkan.
///
/// Contoh:
/// - hanya izinkan GET & POST
/// - block PUT / DELETE / TRACE / dll
pub struct MethodInspector {
    allowed: Vec<Method>,
}

impl MethodInspector {
    /// Buat MethodInspector dengan daftar method yang diizinkan.
    pub fn new(allowed: Vec<Method>) -> Self {
        Self { allowed }
    }
}

impl Inspector for MethodInspector {
    fn inspect(&self, ctx: &RequestContext) -> BulwarkResult<Decision> {
        if self.allowed.contains(&ctx.method) {
            Ok(Decision::Allow)
        } else {
            Err(BulwarkError::blocked(
                "http method not allowed",
            ))
        }
    }
}