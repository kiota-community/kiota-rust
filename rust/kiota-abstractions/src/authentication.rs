use std::collections::{HashMap, HashSet};

use async_trait::async_trait;

use crate::api_error::ApiError;
use crate::request_information::RequestInformation;

/// Trait for authenticating outgoing HTTP requests.
///
/// Implementations add authentication headers (e.g., Bearer tokens, API keys)
/// to a [`RequestInformation`] before it is sent.
#[async_trait]
pub trait AuthenticationProvider: Send + Sync {
    /// Authenticates the provided request by adding necessary headers or parameters.
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
        additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), ApiError>;
}

/// Trait for providing access tokens for authentication.
#[async_trait]
pub trait AccessTokenProvider: Send + Sync {
    /// Returns a bearer token for the given URL.
    async fn get_authorization_token(
        &self,
        url: &url::Url,
        additional_context: Option<&HashMap<String, String>>,
    ) -> Result<String, ApiError>;

    /// Returns the allowed hosts validator for this provider.
    fn get_allowed_hosts_validator(&self) -> &AllowedHostsValidator;
}

/// Validates whether a given URL's host is in the allowed set.
#[derive(Debug, Clone)]
pub struct AllowedHostsValidator {
    allowed_hosts: HashSet<String>,
}

impl AllowedHostsValidator {
    /// Creates a new validator with the given set of allowed hosts.
    pub fn new(hosts: Vec<String>) -> Self {
        Self {
            allowed_hosts: hosts.into_iter().map(|h| h.to_lowercase()).collect(),
        }
    }

    /// Returns `true` if the URL's host is in the allowed set, or if the set is empty.
    pub fn is_url_host_valid(&self, url: &url::Url) -> bool {
        if self.allowed_hosts.is_empty() {
            return true;
        }
        match url.host_str() {
            Some(host) => self.allowed_hosts.contains(&host.to_lowercase()),
            None => false,
        }
    }
}

/// An authentication provider that does not add any authentication.
///
/// Use this for APIs that do not require authentication.
#[derive(Debug, Clone, Default)]
pub struct AnonymousAuthenticationProvider;

#[async_trait]
impl AuthenticationProvider for AnonymousAuthenticationProvider {
    async fn authenticate_request(
        &self,
        _request: &mut RequestInformation,
        _additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), ApiError> {
        Ok(())
    }
}

/// An authentication provider that uses a bearer token from an [`AccessTokenProvider`].
pub struct BaseBearerTokenAuthenticationProvider<T: AccessTokenProvider> {
    access_token_provider: T,
}

impl<T: AccessTokenProvider> BaseBearerTokenAuthenticationProvider<T> {
    /// Creates a new bearer token authentication provider.
    pub fn new(access_token_provider: T) -> Self {
        Self {
            access_token_provider,
        }
    }
}

#[async_trait]
impl<T: AccessTokenProvider> AuthenticationProvider for BaseBearerTokenAuthenticationProvider<T> {
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
        additional_context: Option<&HashMap<String, String>>,
    ) -> Result<(), ApiError> {
        let uri_string = request.get_uri()?;
        let url = url::Url::parse(&uri_string)
            .map_err(|e| ApiError::new(0, format!("Invalid URL: {e}")))?;

        if !self
            .access_token_provider
            .get_allowed_hosts_validator()
            .is_url_host_valid(&url)
        {
            return Ok(());
        }

        let token = self
            .access_token_provider
            .get_authorization_token(&url, additional_context)
            .await?;

        if !token.is_empty() {
            request
                .headers
                .insert("Authorization".to_string(), format!("Bearer {token}"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_hosts_validator_empty_allows_all() {
        let validator = AllowedHostsValidator::new(vec![]);
        let url = url::Url::parse("https://example.com").unwrap();
        assert!(validator.is_url_host_valid(&url));
    }

    #[test]
    fn allowed_hosts_validator_matches_case_insensitive() {
        let validator = AllowedHostsValidator::new(vec!["Graph.Microsoft.Com".to_string()]);
        let url = url::Url::parse("https://graph.microsoft.com/v1.0/me").unwrap();
        assert!(validator.is_url_host_valid(&url));
    }

    #[test]
    fn allowed_hosts_validator_rejects_unknown() {
        let validator = AllowedHostsValidator::new(vec!["graph.microsoft.com".to_string()]);
        let url = url::Url::parse("https://evil.example.com").unwrap();
        assert!(!validator.is_url_host_valid(&url));
    }

    #[tokio::test]
    async fn anonymous_provider_does_nothing() {
        let provider = AnonymousAuthenticationProvider;
        let mut request = RequestInformation::new();
        provider
            .authenticate_request(&mut request, None)
            .await
            .unwrap();
        assert!(!request.headers.contains_key("Authorization"));
    }
}
