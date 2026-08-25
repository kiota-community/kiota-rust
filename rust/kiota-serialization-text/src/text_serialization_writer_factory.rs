//! Factory for [`TextSerializationWriter`].

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::serialization_writer_factory::SerializationWriterFactory;

use crate::text_serialization_writer::TextSerializationWriter;

const CONTENT_TYPE: &str = "text/plain";

pub struct TextSerializationWriterFactory;

impl SerializationWriterFactory for TextSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        CONTENT_TYPE
    }

    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, ApiError> {
        if content_type != CONTENT_TYPE {
            return Err(ApiError::new(
                0,
                format!("Expected {CONTENT_TYPE}, got {content_type}"),
            ));
        }
        Ok(Box::new(TextSerializationWriter::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_writer_for_text() {
        let factory = TextSerializationWriterFactory;
        let writer = factory.get_serialization_writer("text/plain");
        assert!(writer.is_ok());
    }

    #[test]
    fn rejects_wrong_content_type() {
        let factory = TextSerializationWriterFactory;
        let writer = factory.get_serialization_writer("application/json");
        assert!(writer.is_err());
    }
}
