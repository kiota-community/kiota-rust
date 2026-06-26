//! Text/plain [`SerializationWriter`] — writes a single string value.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::time_only::TimeOnly;

/// Writes a single text/plain value.
pub struct TextSerializationWriter {
    value: Option<String>,
}

impl TextSerializationWriter {
    pub fn new() -> Self {
        Self { value: None }
    }
}

impl Default for TextSerializationWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriter for TextSerializationWriter {
    fn write_string_value(&mut self, _key: &str, value: &str) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_bool_value(&mut self, _key: &str, value: bool) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_i32_value(&mut self, _key: &str, value: i32) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_i64_value(&mut self, _key: &str, value: i64) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_f32_value(&mut self, _key: &str, value: f32) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_f64_value(&mut self, _key: &str, value: f64) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_uuid_value(&mut self, _key: &str, value: &Uuid) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_date_only_value(&mut self, _key: &str, value: &DateOnly) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_time_only_value(&mut self, _key: &str, value: &TimeOnly) -> Result<(), ApiError> {
        self.value = Some(value.to_string());
        Ok(())
    }

    fn write_datetime_value(&mut self, _key: &str, value: &DateTime<Utc>) -> Result<(), ApiError> {
        self.value = Some(value.to_rfc3339());
        Ok(())
    }

    fn write_object_value(&mut self, _key: &str, _value: &dyn Parsable) -> Result<(), ApiError> {
        Err(ApiError::new(
            0,
            "Text/plain does not support object serialization".to_string(),
        ))
    }

    fn write_collection_of_object_values(
        &mut self,
        _key: &str,
        _values: &[&dyn Parsable],
    ) -> Result<(), ApiError> {
        Err(ApiError::new(
            0,
            "Text/plain does not support collections".to_string(),
        ))
    }

    fn write_additional_data(&mut self, _data: &HashMap<String, Value>) -> Result<(), ApiError> {
        Ok(())
    }

    fn get_serialized_content(&self) -> Result<Vec<u8>, ApiError> {
        match &self.value {
            Some(v) => Ok(v.as_bytes().to_vec()),
            None => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_string() {
        let mut writer = TextSerializationWriter::new();
        writer.write_string_value("", "hello").unwrap();
        let content = writer.get_serialized_content().unwrap();
        assert_eq!(String::from_utf8(content).unwrap(), "hello");
    }

    #[test]
    fn write_i32() {
        let mut writer = TextSerializationWriter::new();
        writer.write_i32_value("", 42).unwrap();
        let content = writer.get_serialized_content().unwrap();
        assert_eq!(String::from_utf8(content).unwrap(), "42");
    }

    #[test]
    fn empty_writer_returns_empty() {
        let writer = TextSerializationWriter::new();
        let content = writer.get_serialized_content().unwrap();
        assert!(content.is_empty());
    }

    #[test]
    fn write_bool() {
        let mut writer = TextSerializationWriter::new();
        writer.write_bool_value("", true).unwrap();
        let content = writer.get_serialized_content().unwrap();
        assert_eq!(String::from_utf8(content).unwrap(), "true");
    }
}
