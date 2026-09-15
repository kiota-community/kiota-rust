//! HTTP request representation used by Kiota-generated clients.

use std::collections::HashMap;
use std::fmt;

use crate::api_error::ApiError;
use crate::case_insensitive_map::CaseInsensitiveMap;

/// An option that can be attached to a [`RequestInformation`] to influence
/// middleware or handler behaviour.
pub trait RequestOption: Send + Sync {
    /// Returns a unique key identifying this option type.
    fn get_key(&self) -> &str;
}

/// HTTP methods supported by Kiota request infrastructure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// HTTP GET
    Get,
    /// HTTP POST
    Post,
    /// HTTP PUT
    Put,
    /// HTTP PATCH
    Patch,
    /// HTTP DELETE
    Delete,
    /// HTTP OPTIONS
    Options,
    /// HTTP HEAD
    Head,
    /// HTTP TRACE
    Trace,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Head => "HEAD",
            HttpMethod::Trace => "TRACE",
        };
        write!(f, "{s}")
    }
}

/// Describes an HTTP request that will be sent by a [`RequestAdapter`](crate::request_adapter::RequestAdapter).
///
/// Generated code populates this struct; the adapter translates it into an
/// actual HTTP call.
pub struct RequestInformation {
    /// The HTTP method for this request.
    pub http_method: HttpMethod,
    /// A URI template (RFC 6570) for the target resource.
    pub url_template: String,
    /// Path parameters substituted into [`url_template`](Self::url_template).
    pub path_parameters: HashMap<String, String>,
    /// Query parameters appended to the URL.
    pub query_parameters: HashMap<String, String>,
    /// Request headers.
    pub headers: CaseInsensitiveMap,
    /// Optional raw request body content.
    pub content: Option<Vec<u8>>,
    /// Request-scoped options consumed by middleware or handlers.
    pub request_options: Vec<Box<dyn RequestOption>>,
}

impl RequestInformation {
    /// Creates a new `RequestInformation` with sensible defaults (GET, empty
    /// template).
    pub fn new() -> Self {
        Self {
            http_method: HttpMethod::Get,
            url_template: String::new(),
            path_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
            headers: CaseInsensitiveMap::new(),
            content: None,
            request_options: Vec::new(),
        }
    }

    /// Overrides the URL template and clears path/query parameters, using the
    /// supplied URI as-is.
    pub fn set_uri(&mut self, uri: &str) {
        self.url_template = "{+baseurl}".to_string();
        self.path_parameters.clear();
        self.query_parameters.clear();
        self.path_parameters
            .insert("baseurl".to_string(), uri.to_string());
    }

    /// Resolves the final URL from the template and path parameters.
    ///
    /// This performs a simple substitution of `{paramName}` and `{+paramName}`
    /// placeholders. A production implementation would use a full RFC 6570
    /// template engine.
    ///
    /// # Errors
    /// Returns an [`ApiError`] if the URL template is empty.
    pub fn get_uri(&self) -> Result<String, ApiError> {
        if self.url_template.is_empty() {
            return Err(ApiError::new(
                0,
                "URL template is empty — cannot build request URI",
            ));
        }

        let mut url = self.url_template.clone();

        // Replace {+key} and {key} placeholders with path parameter values.
        for (key, value) in &self.path_parameters {
            let placeholder_plus = format!("{{+{key}}}");
            let placeholder = format!("{{{key}}}");
            url = url.replace(&placeholder_plus, value);
            url = url.replace(&placeholder, value);
        }

        // Append query parameters.
        let query_parts: Vec<String> = self
            .query_parameters
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();

        if !query_parts.is_empty() {
            let separator = if url.contains('?') { "&" } else { "?" };
            url.push_str(separator);
            url.push_str(&query_parts.join("&"));
        }

        Ok(url)
    }
}

impl Default for RequestInformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_method_display() {
        assert_eq!(HttpMethod::Get.to_string(), "GET");
        assert_eq!(HttpMethod::Post.to_string(), "POST");
        assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
        assert_eq!(HttpMethod::Patch.to_string(), "PATCH");
        assert_eq!(HttpMethod::Options.to_string(), "OPTIONS");
    }

    #[test]
    fn new_defaults() {
        let req = RequestInformation::new();
        assert_eq!(req.http_method, HttpMethod::Get);
        assert!(req.url_template.is_empty());
        assert!(req.content.is_none());
    }

    #[test]
    fn set_uri_replaces_template() {
        let mut req = RequestInformation::new();
        req.set_uri("https://example.com/api/v1");
        let uri = req.get_uri().unwrap();
        assert_eq!(uri, "https://example.com/api/v1");
    }

    #[test]
    fn get_uri_with_path_params() {
        let mut req = RequestInformation::new();
        req.url_template = "{+baseurl}/users/{userId}".to_string();
        req.path_parameters.insert(
            "baseurl".to_string(),
            "https://graph.microsoft.com".to_string(),
        );
        req.path_parameters
            .insert("userId".to_string(), "123".to_string());
        let uri = req.get_uri().unwrap();
        assert_eq!(uri, "https://graph.microsoft.com/users/123");
    }

    #[test]
    fn get_uri_with_query_params() {
        let mut req = RequestInformation::new();
        req.url_template = "https://api.example.com/items".to_string();
        req.query_parameters
            .insert("top".to_string(), "10".to_string());
        let uri = req.get_uri().unwrap();
        assert!(uri.contains("top=10"));
        assert!(uri.starts_with("https://api.example.com/items?"));
    }

    #[test]
    fn get_uri_empty_template_errors() {
        let req = RequestInformation::new();
        assert!(req.get_uri().is_err());
    }
}
