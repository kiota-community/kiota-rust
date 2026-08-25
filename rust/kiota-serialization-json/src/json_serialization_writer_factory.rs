//! Factory for creating [`JsonSerializationWriter`] instances.

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::serialization_writer_factory::SerializationWriterFactory;

use crate::json_serialization_writer::JsonSerializationWriter;

/// Factory that creates [`JsonSerializationWriter`] instances for `application/json`.
pub struct JsonSerializationWriterFactory;

impl SerializationWriterFactory for JsonSerializationWriterFactory {
    fn valid_content_type(&self) -> &str {
        "application/json"
    }

    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, ApiError> {
        if content_type != self.valid_content_type() {
            return Err(ApiError::new(
                0,
                format!(
                    "expected '{}', got '{content_type}'",
                    self.valid_content_type()
                ),
            ));
        }
        Ok(Box::new(JsonSerializationWriter::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_writer_for_json() {
        let factory = JsonSerializationWriterFactory;
        let writer = factory.get_serialization_writer("application/json");
        assert!(writer.is_ok());
    }

    #[test]
    fn rejects_wrong_content_type() {
        let factory = JsonSerializationWriterFactory;
        let result = factory.get_serialization_writer("text/xml");
        assert!(result.is_err());
    }
}
