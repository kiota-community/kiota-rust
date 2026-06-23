//! reqwest-backed [`RequestAdapter`] implementation.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::authentication::AuthenticationProvider;
use kiota_abstractions::parsable::{Parsable, ParsableFactory};
use kiota_abstractions::parse_node_factory::ParseNodeFactory;
use kiota_abstractions::request_adapter::RequestAdapter;
use kiota_abstractions::request_information::{HttpMethod, RequestInformation};
use kiota_abstractions::serialization_writer_factory::SerializationWriterFactory;

/// An HTTP client request adapter backed by [`reqwest`].
pub struct HttpClientRequestAdapter {
    client: reqwest::Client,
    auth_provider: Arc<dyn AuthenticationProvider>,
    parse_node_factory: Arc<dyn ParseNodeFactory>,
    serialization_writer_factory: Arc<dyn SerializationWriterFactory>,
    base_url: String,
}

impl HttpClientRequestAdapter {
    /// Creates a new adapter with the given authentication provider and factories.
    pub fn new(
        auth_provider: Arc<dyn AuthenticationProvider>,
        parse_node_factory: Arc<dyn ParseNodeFactory>,
        serialization_writer_factory: Arc<dyn SerializationWriterFactory>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth_provider,
            parse_node_factory,
            serialization_writer_factory,
            base_url: String::new(),
        }
    }

    /// Creates a new adapter with a custom reqwest client.
    pub fn with_client(
        client: reqwest::Client,
        auth_provider: Arc<dyn AuthenticationProvider>,
        parse_node_factory: Arc<dyn ParseNodeFactory>,
        serialization_writer_factory: Arc<dyn SerializationWriterFactory>,
    ) -> Self {
        Self {
            client,
            auth_provider,
            parse_node_factory,
            serialization_writer_factory,
            base_url: String::new(),
        }
    }

    /// Builds a reqwest::Request from RequestInformation.
    async fn build_request(
        &self,
        info: &RequestInformation,
    ) -> Result<reqwest::Request, ApiError> {
        let uri = info.get_uri()?;
        let method = match info.http_method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
            HttpMethod::Options => reqwest::Method::OPTIONS,
            HttpMethod::Head => reqwest::Method::HEAD,
            HttpMethod::Trace => reqwest::Method::TRACE,
        };

        let mut builder = self.client.request(method, &uri);

        // Add headers
        for key in info.headers.keys() {
            if let Some(values) = info.headers.get(key) {
                for value in values {
                    builder = builder.header(key, value);
                }
            }
        }

        // Add body
        if let Some(ref content) = info.content {
            builder = builder.body(content.clone());
        }

        builder
            .build()
            .map_err(|e| ApiError::new(0, format!("Failed to build request: {e}")))
    }

    /// Sends the request and returns the response body bytes + content type.
    async fn send_request(
        &self,
        info: &RequestInformation,
    ) -> Result<(Vec<u8>, String), ApiError> {
        // Build a minimal mutable copy for auth (only headers need mutation)
        let mut auth_info = RequestInformation::new();
        auth_info.http_method = info.http_method;
        auth_info.url_template = info.url_template.clone();
        auth_info.path_parameters = info.path_parameters.clone();
        auth_info.query_parameters = info.query_parameters.clone();
        auth_info.headers = info.headers.clone();
        auth_info.content = info.content.clone();

        self.auth_provider
            .authenticate_request(&mut auth_info, None)
            .await?;

        let request = self.build_request(&auth_info).await?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| ApiError::new(0, format!("HTTP request failed: {e}")))?;

        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/json")
            .split(';')
            .next()
            .unwrap_or("application/json")
            .trim()
            .to_string();

        if status >= 400 {
            let body = response.text().await.unwrap_or_default();
            return Err(ApiError {
                status_code: status as i32,
                message: format!("HTTP {status}"),
                headers: HashMap::new(),
                response_body: Some(body),
            });
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ApiError::new(0, format!("Failed to read response body: {e}")))?;

        Ok((bytes.to_vec(), content_type))
    }
}

#[async_trait]
impl RequestAdapter for HttpClientRequestAdapter {
    async fn send<T: Parsable + Default + 'static>(
        &self,
        info: &RequestInformation,
        _error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Option<T>, ApiError> {
        let (bytes, content_type) = self.send_request(info).await?;
        if bytes.is_empty() {
            return Ok(None);
        }
        let node = self
            .parse_node_factory
            .get_root_parse_node(&content_type, &bytes)?;
        let factory = || -> Box<dyn Parsable> { Box::new(T::default()) };
        let result = node.get_object_value(&factory)?;
        match result {
            Some(boxed) => {
                // Downcast from Box<dyn Parsable> to T
                match boxed.as_any().downcast_ref::<T>() {
                    Some(_) => {
                        // We need to move out of the box, so reconstruct
                        // This is a limitation — for now return default + populated fields
                        Ok(None) // TODO: proper downcast with Box<dyn Any>
                    }
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    async fn send_collection<T: Parsable + Default + 'static>(
        &self,
        info: &RequestInformation,
        _error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Vec<T>, ApiError> {
        let (bytes, content_type) = self.send_request(info).await?;
        if bytes.is_empty() {
            return Ok(Vec::new());
        }
        let node = self
            .parse_node_factory
            .get_root_parse_node(&content_type, &bytes)?;
        let factory = || -> Box<dyn Parsable> { Box::new(T::default()) };
        let _results = node.get_collection_of_object_values(&factory)?;
        // TODO: downcast each item from Box<dyn Parsable> to T
        Ok(Vec::new())
    }

    async fn send_primitive<T: Send + Sync + 'static>(
        &self,
        info: &RequestInformation,
        _error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Option<T>, ApiError> {
        let (_bytes, _content_type) = self.send_request(info).await?;
        // TODO: implement primitive deserialization
        Ok(None)
    }

    async fn send_no_content(
        &self,
        info: &RequestInformation,
        _error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<(), ApiError> {
        let _ = self.send_request(info).await?;
        Ok(())
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn set_base_url(&mut self, url: &str) {
        self.base_url = url.to_string();
    }

    fn get_serialization_writer_factory(&self) -> &dyn SerializationWriterFactory {
        self.serialization_writer_factory.as_ref()
    }
}
