//! Form URL-encoded [`SerializationWriter`].

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::time_only::TimeOnly;

/// Writes key-value pairs as `application/x-www-form-urlencoded`.
pub struct FormSerializationWriter {
    pairs: Vec<(String, String)>,
}

impl FormSerializationWriter {
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }
}

impl Default for FormSerializationWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriter for FormSerializationWriter {
    fn write_string_value(&mut self, key: &str, value: &str) -> Result<(), ApiError> {
        self.pairs.push((key.to_string(), value.to_string()));
        Ok(())
    }

    fn write_bool_value(&mut self, key: &str, value: bool) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_i32_value(&mut self, key: &str, value: i32) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_i64_value(&mut self, key: &str, value: i64) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_f32_value(&mut self, key: &str, value: f32) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_f64_value(&mut self, key: &str, value: f64) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_uuid_value(&mut self, key: &str, value: &Uuid) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_date_only_value(&mut self, key: &str, value: &DateOnly) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_time_only_value(&mut self, key: &str, value: &TimeOnly) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_string())
    }

    fn write_datetime_value(&mut self, key: &str, value: &DateTime<Utc>) -> Result<(), ApiError> {
        self.write_string_value(key, &value.to_rfc3339())
    }

    fn write_object_value(&mut self, _key: &str, value: &dyn Parsable) -> Result<(), ApiError> {
        let mut child = FormSerializationWriter::new();
        value.serialize(&mut child)?;
        self.pairs.extend(child.pairs);
        Ok(())
    }

    fn write_collection_of_object_values(
        &mut self,
        _key: &str,
        _values: &[&dyn Parsable],
    ) -> Result<(), ApiError> {
        Err(ApiError::new(
            0,
            "Form encoding does not support collections".to_string(),
        ))
    }

    fn write_additional_data(&mut self, data: &HashMap<String, Value>) -> Result<(), ApiError> {
        for (key, val) in data {
            match val {
                Value::String(s) => self.write_string_value(key, s)?,
                other => self.write_string_value(key, &other.to_string())?,
            }
        }
        Ok(())
    }

    fn get_serialized_content(&self) -> Result<Vec<u8>, ApiError> {
        let encoded: String = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.pairs)
            .finish();
        Ok(encoded.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_single_pair() {
        let mut writer = FormSerializationWriter::new();
        writer.write_string_value("name", "Alice").unwrap();
        let content = String::from_utf8(writer.get_serialized_content().unwrap()).unwrap();
        assert_eq!(content, "name=Alice");
    }

    #[test]
    fn write_multiple_pairs() {
        let mut writer = FormSerializationWriter::new();
        writer.write_string_value("name", "Alice").unwrap();
        writer.write_i32_value("age", 30).unwrap();
        let content = String::from_utf8(writer.get_serialized_content().unwrap()).unwrap();
        assert_eq!(content, "name=Alice&age=30");
    }

    #[test]
    fn encodes_special_chars() {
        let mut writer = FormSerializationWriter::new();
        writer
            .write_string_value("msg", "hello world & more")
            .unwrap();
        let content = String::from_utf8(writer.get_serialized_content().unwrap()).unwrap();
        assert!(content.contains("hello+world"));
    }

    #[test]
    fn empty_writer() {
        let writer = FormSerializationWriter::new();
        let content = writer.get_serialized_content().unwrap();
        assert!(content.is_empty());
    }
}
