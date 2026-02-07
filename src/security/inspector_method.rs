use crate::request::context::{RequestContext, Method};
use crate::security::inspector::{Inspector, InspectorFinding};
use crate::security::decision::FindingSeverity;
use crate::BulwarkError;

pub struct InspectorMethod {
    allowed_methods: Vec<Method>,
}

impl InspectorMethod {
    pub fn new(allowed_methods: Vec<Method>) -> Self {
        Self { allowed_methods }
    }
}

impl Inspector for InspectorMethod {
    fn inspect(
        &self,
        ctx: &RequestContext,
    ) -> Result<Option<InspectorFinding>, BulwarkError> {
        if !self.allowed_methods.contains(&ctx.method) {
            return Ok(Some(InspectorFinding::new(
                "inspector_method",
                FindingSeverity::High,
                format!("method {:?} is not allowed", ctx.method),
            )));
        }

        Ok(None)
    }
}