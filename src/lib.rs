//! Bulwark
//!
//! A security-first backend framework for Rust.
//!
//! Philosophy:
//! - Secure by default
//! - Explicit request decisions
//! - Minimal magic
//! - Defense comes first

pub mod logging;
pub mod request;
pub mod security;
pub mod server;

/// Core decision made by Bulwark security pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Block,
    Log,
}

/// Result type used across Bulwark.
pub type BulwarkResult<T> = Result<T, BulwarkError>;

/// Global error type for Bulwark.
#[derive(Debug)]
pub enum BulwarkError {
    Blocked { reason: &'static str },
    InternalError { message: &'static str },
}

impl BulwarkError {
    pub fn blocked(reason: &'static str) -> Self {
        BulwarkError::Blocked { reason }
    }

    pub fn internal(message: &'static str) -> Self {
        BulwarkError::InternalError { message }
    }
}
