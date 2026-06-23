use std::collections::HashMap;

use async_trait::async_trait;

use crate::Parsable;
use crate::api_error::ApiError;
use crate::parsable::ParsableFactory;
use crate::request_information::RequestInformation;
use crate::serialization_writer_factory::SerializationWriterFactory;

/// Trait for executing generated request builders.
///
/// This is the main abstraction that Kiota-generated code calls to make HTTP
/// requests. Implementations handle serialization, deserialization, middleware,
/// and authentication.
#[async_trait]
pub trait RequestAdapter: Send + Sync {
    /// Sends a request that returns a single object.
    async fn send<T: Parsable + Default + 'static>(
        &self,
        info: &RequestInformation,
        error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Option<T>, ApiError>;

    /// Sends a request that returns a collection of objects.
    async fn send_collection<T: Parsable + Default + 'static>(
        &self,
        info: &RequestInformation,
        error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Vec<T>, ApiError>;

    /// Sends a request that returns a primitive value.
    async fn send_primitive<T: Send + Sync + 'static>(
        &self,
        info: &RequestInformation,
        error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<Option<T>, ApiError>;

    /// Sends a request with no response body.
    async fn send_no_content(
        &self,
        info: &RequestInformation,
        error_mappings: Option<&HashMap<String, ParsableFactory>>,
    ) -> Result<(), ApiError>;

    /// Returns the base URL for all requests.
    fn get_base_url(&self) -> &str;

    /// Sets the base URL for all requests.
    fn set_base_url(&mut self, url: &str);

    /// Returns the serialization writer factory used by this adapter.
    fn get_serialization_writer_factory(&self) -> &dyn SerializationWriterFactory;
}
