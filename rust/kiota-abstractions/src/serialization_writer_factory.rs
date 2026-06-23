//! Factory trait and registry for creating [`SerializationWriter`] instances.

use std::collections::HashMap;
use std::sync::Arc;

use crate::api_error::ApiError;
use crate::serialization_writer::SerializationWriter;

/// Creates [`SerializationWriter`] instances for a specific content type.
pub trait SerializationWriterFactory: Send + Sync {
    /// Returns the MIME content type this factory handles (e.g. `"application/json"`).
    fn valid_content_type(&self) -> &str;

    /// Creates a new [`SerializationWriter`] for the given content type.
    ///
    /// # Errors
    /// Returns an [`ApiError`] if `content_type` is unsupported.
    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, ApiError>;
}

/// A registry that multiplexes [`SerializationWriterFactory`] implementations
/// by content type.
///
/// Register format-specific factories via
/// [`register`](SerializationWriterFactoryRegistry::register), then use the
/// registry as a single [`SerializationWriterFactory`] that delegates to the
/// correct implementation.
#[derive(Default)]
pub struct SerializationWriterFactoryRegistry {
    factories: HashMap<String, Arc<dyn SerializationWriterFactory>>,
}

impl SerializationWriterFactoryRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a factory for its declared content type.
    ///
    /// If a factory was already registered for the same content type, it is
    /// replaced.
    pub fn register(&mut self, factory: Arc<dyn SerializationWriterFactory>) {
        let content_type = factory.valid_content_type().to_owned();
        self.factories.insert(content_type, factory);
    }

    /// Returns the factory registered for `content_type`, if any.
    pub fn get(&self, content_type: &str) -> Option<&Arc<dyn SerializationWriterFactory>> {
        self.factories.get(content_type)
    }
}

impl SerializationWriterFactory for SerializationWriterFactoryRegistry {
    /// Returns an empty string because the registry supports multiple content
    /// types.
    fn valid_content_type(&self) -> &str {
        ""
    }

    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, ApiError> {
        let factory = self.factories.get(content_type).ok_or_else(|| {
            ApiError::new(
                0,
                format!(
                    "no serialization writer factory registered for content type '{content_type}'"
                ),
            )
        })?;
        factory.get_serialization_writer(content_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_returns_error_for_unknown_content_type() {
        let registry = SerializationWriterFactoryRegistry::new();
        let result = registry.get_serialization_writer("application/xml");
        match result {
            Err(err) => assert!(err.message.contains("application/xml")),
            Ok(_) => panic!("expected error for unknown content type"),
        }
    }

    #[test]
    fn valid_content_type_returns_empty() {
        let registry = SerializationWriterFactoryRegistry::new();
        assert_eq!(registry.valid_content_type(), "");
    }
}
