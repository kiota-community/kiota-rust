//! JSON-backed implementation of [`ParseNode`].

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::time_only::TimeOnly;

/// A [`ParseNode`] implementation backed by a [`serde_json::Value`].
pub struct JsonParseNode {
    value: Value,
}

impl JsonParseNode {
    /// Creates a new `JsonParseNode` wrapping the given JSON value.
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    /// Parses raw bytes into a `JsonParseNode`.
    pub fn from_bytes(content: &[u8]) -> Result<Self, ApiError> {
        let value: Value = serde_json::from_slice(content)
            .map_err(|e| ApiError::new(0, format!("JSON parse error: {e}")))?;
        Ok(Self { value })
    }
}

impl ParseNode for JsonParseNode {
    fn get_string_value(&self) -> Result<Option<String>, ApiError> {
        Ok(self.value.as_str().map(|s| s.to_string()))
    }

    fn get_bool_value(&self) -> Result<Option<bool>, ApiError> {
        Ok(self.value.as_bool())
    }

    fn get_i32_value(&self) -> Result<Option<i32>, ApiError> {
        Ok(self.value.as_i64().and_then(|v| i32::try_from(v).ok()))
    }

    fn get_i64_value(&self) -> Result<Option<i64>, ApiError> {
        Ok(self.value.as_i64())
    }

    fn get_f32_value(&self) -> Result<Option<f32>, ApiError> {
        Ok(self.value.as_f64().map(|v| v as f32))
    }

    fn get_f64_value(&self) -> Result<Option<f64>, ApiError> {
        Ok(self.value.as_f64())
    }

    fn get_uuid_value(&self) -> Result<Option<Uuid>, ApiError> {
        match self.value.as_str() {
            Some(s) => {
                let uuid = Uuid::parse_str(s)
                    .map_err(|e| ApiError::new(0, format!("Invalid UUID: {e}")))?;
                Ok(Some(uuid))
            }
            None => Ok(None),
        }
    }

    fn get_date_only_value(&self) -> Result<Option<DateOnly>, ApiError> {
        match self.value.as_str() {
            Some(s) => {
                let date: DateOnly = s
                    .parse()
                    .map_err(|e| ApiError::new(0, format!("Invalid date: {e}")))?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }

    fn get_time_only_value(&self) -> Result<Option<TimeOnly>, ApiError> {
        match self.value.as_str() {
            Some(s) => {
                let time: TimeOnly = s
                    .parse()
                    .map_err(|e| ApiError::new(0, format!("Invalid time: {e}")))?;
                Ok(Some(time))
            }
            None => Ok(None),
        }
    }

    fn get_datetime_value(&self) -> Result<Option<DateTime<Utc>>, ApiError> {
        match self.value.as_str() {
            Some(s) => {
                let dt = s
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| ApiError::new(0, format!("Invalid datetime: {e}")))?;
                Ok(Some(dt))
            }
            None => Ok(None),
        }
    }

    fn get_enum_value(
        &self,
        parser: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Option<String>, ApiError> {
        match self.value.as_str() {
            Some(s) => Ok(parser(s)),
            None => Ok(None),
        }
    }

    fn get_object_value(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Option<Box<dyn Parsable>>, ApiError> {
        if !self.value.is_object() {
            return Ok(None);
        }
        let mut model = factory();
        let deserializers = model.get_field_deserializers();
        if let Value::Object(map) = &self.value {
            for (key, val) in map {
                if let Some(deserializer) = deserializers.get(key.as_str()) {
                    let child_node = JsonParseNode::new(val.clone());
                    deserializer(model.as_mut(), &child_node);
                }
            }
        }
        Ok(Some(model))
    }

    fn get_collection_of_object_values(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Vec<Box<dyn Parsable>>, ApiError> {
        let mut results = Vec::new();
        if let Value::Array(arr) = &self.value {
            for item in arr {
                let node = JsonParseNode::new(item.clone());
                if let Some(obj) = node.get_object_value(factory)? {
                    results.push(obj);
                }
            }
        }
        Ok(results)
    }

    fn get_collection_of_primitive_values(&self) -> Result<Vec<String>, ApiError> {
        let mut results = Vec::new();
        if let Value::Array(arr) = &self.value {
            for item in arr {
                match item {
                    Value::String(s) => results.push(s.clone()),
                    Value::Number(n) => results.push(n.to_string()),
                    Value::Bool(b) => results.push(b.to_string()),
                    _ => {}
                }
            }
        }
        Ok(results)
    }

    fn get_child_node(&self, key: &str) -> Option<Box<dyn ParseNode>> {
        self.value
            .get(key)
            .map(|v| Box::new(JsonParseNode::new(v.clone())) as Box<dyn ParseNode>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_string() {
        let node = JsonParseNode::new(json!("hello"));
        assert_eq!(node.get_string_value().unwrap(), Some("hello".to_string()));
    }

    #[test]
    fn parse_bool() {
        let node = JsonParseNode::new(json!(true));
        assert_eq!(node.get_bool_value().unwrap(), Some(true));
    }

    #[test]
    fn parse_i32() {
        let node = JsonParseNode::new(json!(42));
        assert_eq!(node.get_i32_value().unwrap(), Some(42));
    }

    #[test]
    fn parse_f64() {
        let node = JsonParseNode::new(json!(3.14));
        let val = node.get_f64_value().unwrap().unwrap();
        assert!((val - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_null_returns_none() {
        let node = JsonParseNode::new(json!(null));
        assert_eq!(node.get_string_value().unwrap(), None);
        assert_eq!(node.get_bool_value().unwrap(), None);
        assert_eq!(node.get_i32_value().unwrap(), None);
    }

    #[test]
    fn parse_uuid() {
        let id = "550e8400-e29b-41d4-a716-446655440000";
        let node = JsonParseNode::new(json!(id));
        let parsed = node.get_uuid_value().unwrap().unwrap();
        assert_eq!(parsed.to_string(), id);
    }

    #[test]
    fn parse_date_only() {
        let node = JsonParseNode::new(json!("2026-06-23"));
        let date = node.get_date_only_value().unwrap().unwrap();
        assert_eq!(date.to_string(), "2026-06-23");
    }

    #[test]
    fn parse_child_node() {
        let node = JsonParseNode::new(json!({"name": "kiota", "version": 1}));
        let child = node.get_child_node("name").unwrap();
        assert_eq!(child.get_string_value().unwrap(), Some("kiota".to_string()));
    }

    #[test]
    fn parse_collection_of_primitives() {
        let node = JsonParseNode::new(json!(["a", "b", "c"]));
        let vals = node.get_collection_of_primitive_values().unwrap();
        assert_eq!(vals, vec!["a", "b", "c"]);
    }

    #[test]
    fn from_bytes_valid_json() {
        let bytes = b"{\"key\": \"value\"}";
        let node = JsonParseNode::from_bytes(bytes).unwrap();
        let child = node.get_child_node("key").unwrap();
        assert_eq!(child.get_string_value().unwrap(), Some("value".to_string()));
    }

    #[test]
    fn from_bytes_invalid_json() {
        let bytes = b"not json";
        let result = JsonParseNode::from_bytes(bytes);
        assert!(result.is_err());
    }
}
