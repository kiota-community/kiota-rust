//! Middleware pipeline for the HTTP client.
//!
//! Placeholder for retry, redirect, and telemetry middleware.
//! These will be implemented in follow-up PRs.

/// Trait for HTTP middleware handlers.
pub trait Middleware: Send + Sync {
    /// Process the request, optionally delegating to the next handler.
    fn send(
        &self,
        request: reqwest::Request,
        next: &dyn Middleware,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> + Send>,
    >;
}
