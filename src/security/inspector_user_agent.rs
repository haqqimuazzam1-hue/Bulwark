use crate::request::context::RequestContext;
use crate::security::inspector::{Inspector, InspectorFinding};
use crate::security::decision::FindingSeverity;
use crate::BulwarkError;

/// InspectorUserAgent
///
/// Mengecek User-Agent yang mencurigakan.
/// Heuristic sederhana:
/// - Kosong / tidak ada -> Medium
/// - Mengandung keyword berisiko -> Medium
/// - Terlalu panjang -> High
pub struct InspectorUserAgent {
    /// Panjang maksimum UA yang masih dianggap wajar
    max_length: usize,

    /// Daftar keyword mencurigakan (lowercase)
    suspicious_keywords: Vec<&'static str>,
}

impl InspectorUserAgent {
    pub fn new(max_length: usize, suspicious_keywords: Vec<&'static str>) -> Self {
        Self {
            max_length,
            suspicious_keywords,
        }
    }
}

impl Inspector for InspectorUserAgent {
    fn inspect(
        &self,
        ctx: &RequestContext,
    ) -> Result<Option<InspectorFinding>, BulwarkError> {
        let ua = match ctx.headers.get("user-agent") {
            Some(v) => v,
            None => {
                return Ok(Some(InspectorFinding::new(
                    "inspector_user_agent",
                    FindingSeverity::Medium,
                    "missing user-agent header",
                )));
            }
        };

        // Terlalu panjang -> High
        if ua.len() > self.max_length {
            return Ok(Some(InspectorFinding::new(
                "inspector_user_agent",
                FindingSeverity::High,
                format!(
                    "user-agent length {} exceeds max {}",
                    ua.len(),
                    self.max_length
                ),
            )));
        }

        let ua_lower = ua.to_lowercase();

        // Mengandung keyword mencurigakan -> Medium
        for keyword in &self.suspicious_keywords {
            if ua_lower.contains(keyword) {
                return Ok(Some(InspectorFinding::new(
                    "inspector_user_agent",
                    FindingSeverity::Medium,
                    format!("user-agent contains suspicious keyword `{}`", keyword),
                )));
            }
        }

        // Aman
        Ok(None)
    }
}