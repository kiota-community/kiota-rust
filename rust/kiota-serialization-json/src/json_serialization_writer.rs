//! JSON-backed implementation of [`SerializationWriter`].

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::time_only::TimeOnly;

/// A [`SerializationWriter`] that produces JSON output via [`serde_json`].
pub struct JsonSerializationWriter {
    root: Map<String, Value>,
    /// Used when writing a value without a key (e.g., collection items).
    current_value: Option<Value>,
}

impl JsonSerializationWriter {
    /// Creates a new empty JSON writer.
    pub fn new() -> Self {
        Self {
            root: Map::new(),
            current_value: None,
        }
    }
}

impl Default for JsonSerializationWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationWriter for JsonSerializationWriter {
    fn write_string_value(&mut self, key: &str, value: &str) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::String(value.to_string()));
        Ok(())
    }

    fn write_bool_value(&mut self, key: &str, value: bool) -> Result<(), ApiError> {
        self.root.insert(key.to_string(), Value::Bool(value));
        Ok(())
    }

    fn write_i32_value(&mut self, key: &str, value: i32) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::Number(value.into()));
        Ok(())
    }

    fn write_i64_value(&mut self, key: &str, value: i64) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::Number(value.into()));
        Ok(())
    }

    fn write_f32_value(&mut self, key: &str, value: f32) -> Result<(), ApiError> {
        let num = serde_json::Number::from_f64(value as f64)
            .ok_or_else(|| ApiError::new(0, format!("Cannot serialize f32: {value}")))?;
        self.root.insert(key.to_string(), Value::Number(num));
        Ok(())
    }

    fn write_f64_value(&mut self, key: &str, value: f64) -> Result<(), ApiError> {
        let num = serde_json::Number::from_f64(value)
            .ok_or_else(|| ApiError::new(0, format!("Cannot serialize f64: {value}")))?;
        self.root.insert(key.to_string(), Value::Number(num));
        Ok(())
    }

    fn write_uuid_value(&mut self, key: &str, value: &Uuid) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::String(value.to_string()));
        Ok(())
    }

    fn write_date_only_value(&mut self, key: &str, value: &DateOnly) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::String(value.to_string()));
        Ok(())
    }

    fn write_time_only_value(&mut self, key: &str, value: &TimeOnly) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::String(value.to_string()));
        Ok(())
    }

    fn write_datetime_value(&mut self, key: &str, value: &DateTime<Utc>) -> Result<(), ApiError> {
        self.root
            .insert(key.to_string(), Value::String(value.to_rfc3339()));
        Ok(())
    }

    fn write_object_value(&mut self, key: &str, value: &dyn Parsable) -> Result<(), ApiError> {
        let mut child_writer = JsonSerializationWriter::new();
        value.serialize(&mut child_writer)?;
        self.root
            .insert(key.to_string(), Value::Object(child_writer.root));
        Ok(())
    }

    fn write_collection_of_object_values(
        &mut self,
        key: &str,
        values: &[&dyn Parsable],
    ) -> Result<(), ApiError> {
        let mut arr = Vec::new();
        for item in values {
            let mut child_writer = JsonSerializationWriter::new();
            item.serialize(&mut child_writer)?;
            arr.push(Value::Object(child_writer.root));
        }
        self.root.insert(key.to_string(), Value::Array(arr));
        Ok(())
    }

    fn write_additional_data(&mut self, data: &HashMap<String, Value>) -> Result<(), ApiError> {
        for (key, val) in data {
            self.root.insert(key.clone(), val.clone());
        }
        Ok(())
    }

    fn get_serialized_content(&self) -> Result<Vec<u8>, ApiError> {
        let value = if let Some(ref v) = self.current_value {
            v.clone()
        } else {
            Value::Object(self.root.clone())
        };
        serde_json::to_vec(&value)
            .map_err(|e| ApiError::new(0, format!("JSON serialization error: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn write_string_and_serialize() {
        let mut writer = JsonSerializationWriter::new();
        writer.write_string_value("name", "kiota").unwrap();
        let bytes = writer.get_serialized_content().unwrap();
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed["name"], "kiota");
    }

    #[test]
    fn write_multiple_types() {
        let mut writer = JsonSerializationWriter::new();
        writer.write_string_value("name", "test").unwrap();
        writer.write_bool_value("active", true).unwrap();
        writer.write_i32_value("count", 42).unwrap();
        writer.write_f64_value("score", 9.5).unwrap();

        let bytes = writer.get_serialized_content().unwrap();
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(parsed["name"], "test");
        assert_eq!(parsed["active"], true);
        assert_eq!(parsed["count"], 42);
        assert_eq!(parsed["score"], 9.5);
    }

    #[test]
    fn write_uuid() {
        let mut writer = JsonSerializationWriter::new();
        let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        writer.write_uuid_value("id", &id).unwrap();
        let bytes = writer.get_serialized_content().unwrap();
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed["id"], "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn write_additional_data() {
        let mut writer = JsonSerializationWriter::new();
        let mut data = HashMap::new();
        data.insert("extra".to_string(), Value::String("info".to_string()));
        data.insert("count".to_string(), Value::Number(7.into()));
        writer.write_additional_data(&data).unwrap();

        let bytes = writer.get_serialized_content().unwrap();
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed["extra"], "info");
        assert_eq!(parsed["count"], 7);
    }

    #[test]
    fn empty_writer_produces_empty_object() {
        let writer = JsonSerializationWriter::new();
        let bytes = writer.get_serialized_content().unwrap();
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed, json!({}));
    }
}
