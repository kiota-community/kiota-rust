//! Factory for [`FormSerializationWriter`].

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::serialization_writer_factory::SerializationWriterFactory;

use crate::form_serialization_writer::FormSerializationWriter;

const CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

pub struct FormSerializationWriterFactory;

impl SerializationWriterFactory for FormSerializationWriterFactory {
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
        Ok(Box::new(FormSerializationWriter::new()))
    }
}
