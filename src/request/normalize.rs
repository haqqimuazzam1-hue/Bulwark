use super::context::RequestContext;

/// Normalize request context before inspection.
///
/// Goals:
/// - predictable input
/// - no ambiguity
/// - reduce bypass surface
pub struct Normalizer;

impl Normalizer {
    /// Run all normalization steps.
    pub fn normalize(ctx: &mut RequestContext) {
        Self::normalize_path(ctx);
        Self::normalize_headers(ctx);
        Self::normalize_query(ctx);
    }

    /// Normalize request path.
    ///
    /// - remove trailing slashes (except root)
    /// - collapse multiple slashes
    fn normalize_path(ctx: &mut RequestContext) {
        let mut path = ctx.path.clone();

        // collapse multiple slashes: ///a//b -> /a/b
        while path.contains("//") {
            path = path.replace("//", "/");
        }

        // remove trailing slash (except "/")
        if path.len() > 1 && path.ends_with('/') {
            path.pop();
        }

        ctx.path = path;
    }

    /// Normalize headers.
    ///
    /// - trim whitespace
    /// - lowercase keys (already done, but re-assert)
    fn normalize_headers(ctx: &mut RequestContext) {
        let mut normalized = std::collections::HashMap::new();

        for (k, v) in ctx.headers.iter() {
            let key = k.trim().to_lowercase();
            let value = v.trim().to_string();
            normalized.insert(key, value);
        }

        ctx.headers = normalized;
    }

    /// Normalize query parameters.
    ///
    /// - trim whitespace
    /// - drop empty keys
    fn normalize_query(ctx: &mut RequestContext) {
        let mut normalized = std::collections::HashMap::new();

        for (k, v) in ctx.query.iter() {
            let key = k.trim();
            if key.is_empty() {
                continue;
            }

            normalized.insert(key.to_string(), v.trim().to_string());
        }

        ctx.query = normalized;
    }
}