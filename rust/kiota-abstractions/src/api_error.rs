//! Error type for API-level failures returned by Kiota-generated clients.

use std::collections::HashMap;
use std::fmt;

/// Represents an error returned by an API endpoint.
///
/// Contains the HTTP status code, a human-readable message, any response
/// headers, and an optional snapshot of the response body.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// HTTP status code received from the API.
    pub status_code: i32,
    /// Human-readable error message.
    pub message: String,
    /// Response headers associated with the error.
    pub headers: HashMap<String, Vec<String>>,
    /// Optional raw response body, when available.
    pub response_body: Option<String>,
}

impl ApiError {
    /// Creates a new `ApiError` with the given status code and message.
    ///
    /// Headers default to an empty map and `response_body` defaults to `None`.
    pub fn new(status_code: i32, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
            headers: HashMap::new(),
            response_body: None,
        }
    }

    /// Builder-style setter for the response body.
    pub fn with_response_body(mut self, body: impl Into<String>) -> Self {
        self.response_body = Some(body.into());
        self
    }

    /// Builder-style setter for headers.
    pub fn with_headers(mut self, headers: HashMap<String, Vec<String>>) -> Self {
        self.headers = headers;
        self
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API error {}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for ApiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_error_with_defaults() {
        let err = ApiError::new(404, "Not Found");
        assert_eq!(err.status_code, 404);
        assert_eq!(err.message, "Not Found");
        assert!(err.headers.is_empty());
        assert!(err.response_body.is_none());
    }

    #[test]
    fn display_format() {
        let err = ApiError::new(500, "Internal Server Error");
        assert_eq!(format!("{err}"), "API error 500: Internal Server Error");
    }

    #[test]
    fn with_response_body_sets_body() {
        let err = ApiError::new(400, "Bad Request")
            .with_response_body("{\"error\": \"invalid\"}");
        assert_eq!(
            err.response_body.as_deref(),
            Some("{\"error\": \"invalid\"}")
        );
    }

    #[test]
    fn implements_error_trait() {
        let err: Box<dyn std::error::Error> =
            Box::new(ApiError::new(503, "Service Unavailable"));
        assert!(err.to_string().contains("503"));
    }
}
