//! Factory trait and registry for creating [`ParseNode`] instances from content.

use std::collections::HashMap;
use std::sync::Arc;

use crate::api_error::ApiError;
use crate::parse_node::ParseNode;

/// Creates [`ParseNode`] instances from raw byte content of a specific type.
pub trait ParseNodeFactory: Send + Sync {
    /// Returns the MIME content type this factory handles (e.g. `"application/json"`).
    fn valid_content_type(&self) -> &str;

    /// Parses `content` and returns the root [`ParseNode`].
    ///
    /// # Errors
    /// Returns an [`ApiError`] if `content_type` is unsupported or parsing fails.
    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, ApiError>;
}

/// A registry that multiplexes [`ParseNodeFactory`] implementations by content
/// type.
///
/// Register format-specific factories via [`register`](ParseNodeFactoryRegistry::register),
/// then use it as a single [`ParseNodeFactory`] that delegates to the correct
/// implementation based on the requested content type.
#[derive(Default)]
pub struct ParseNodeFactoryRegistry {
    factories: HashMap<String, Arc<dyn ParseNodeFactory>>,
}

impl ParseNodeFactoryRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a factory for its declared content type.
    ///
    /// If a factory was already registered for the same content type, it is
    /// replaced.
    pub fn register(&mut self, factory: Arc<dyn ParseNodeFactory>) {
        let content_type = factory.valid_content_type().to_owned();
        self.factories.insert(content_type, factory);
    }

    /// Returns the factory registered for `content_type`, if any.
    pub fn get(&self, content_type: &str) -> Option<&Arc<dyn ParseNodeFactory>> {
        self.factories.get(content_type)
    }
}

impl ParseNodeFactory for ParseNodeFactoryRegistry {
    /// Returns an empty string because the registry supports multiple content
    /// types.
    fn valid_content_type(&self) -> &str {
        ""
    }

    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, ApiError> {
        let factory = self.factories.get(content_type).ok_or_else(|| {
            ApiError::new(
                0,
                format!("no parse node factory registered for content type '{content_type}'"),
            )
        })?;
        factory.get_root_parse_node(content_type, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_returns_error_for_unknown_content_type() {
        let registry = ParseNodeFactoryRegistry::new();
        let result = registry.get_root_parse_node("application/xml", &[]);
        match result {
            Err(err) => assert!(err.message.contains("application/xml")),
            Ok(_) => panic!("expected error for unknown content type"),
        }
    }

    #[test]
    fn valid_content_type_returns_empty() {
        let registry = ParseNodeFactoryRegistry::new();
        assert_eq!(registry.valid_content_type(), "");
    }
}
