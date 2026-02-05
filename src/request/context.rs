use std::collections::HashMap;

/// Represents a normalized HTTP request inside Bulwark.
///
/// This is NOT a raw HTTP request.
/// Everything here is meant to be:
/// - explicit
/// - predictable
/// - safe to inspect
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// HTTP method (GET, POST, etc)
    pub method: Method,

    /// Normalized path (no query string)
    pub path: String,

    /// Query parameters (?a=b&c=d)
    pub query: HashMap<String, String>,

    /// Request headers (lowercased keys)
    pub headers: HashMap<String, String>,

    /// Raw request body (optional)
    pub body: Option<Vec<u8>>,

    /// Client IP (if known)
    pub client_ip: Option<String>,
}

impl RequestContext {
    /// Create a new empty request context.
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            client_ip: None,
        }
    }

    /// Insert a header (key will be lowercased).
    pub fn insert_header(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.headers
            .insert(key.into().to_lowercase(), value.into());
    }

    /// Insert a query parameter.
    pub fn insert_query(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.query.insert(key.into(), value.into());
    }

    /// Set request body.
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }

    /// Set client IP.
    pub fn set_client_ip(&mut self, ip: impl Into<String>) {
        self.client_ip = Some(ip.into());
    }

    /// Get a header value (case-insensitive).
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }

    /// Check if request has a body.
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
}

/// Supported HTTP methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Method {
    /// Convert from &str safely.
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "PATCH" => Some(Method::PATCH),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            _ => None,
        }
    }
}