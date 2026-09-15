//! Form URL-encoded [`ParseNode`] implementation.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::time_only::TimeOnly;

/// A [`ParseNode`] backed by URL-encoded form fields.
pub struct FormParseNode {
    fields: HashMap<String, String>,
    current_value: Option<String>,
}

impl FormParseNode {
    pub fn new(fields: HashMap<String, String>) -> Self {
        Self {
            fields,
            current_value: None,
        }
    }

    pub fn from_bytes(content: &[u8]) -> Result<Self, ApiError> {
        let s = std::str::from_utf8(content)
            .map_err(|e| ApiError::new(0, format!("Invalid UTF-8: {e}")))?;
        let fields: HashMap<String, String> = url::form_urlencoded::parse(s.as_bytes())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        Ok(Self::new(fields))
    }

    fn current_str(&self) -> Option<&str> {
        self.current_value.as_deref()
    }
}

impl ParseNode for FormParseNode {
    fn get_string_value(&self) -> Result<Option<String>, ApiError> {
        Ok(self.current_str().map(|s| s.to_string()))
    }

    fn get_bool_value(&self) -> Result<Option<bool>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<bool>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse bool: {e}"))),
            None => Ok(None),
        }
    }

    fn get_i32_value(&self) -> Result<Option<i32>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<i32>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse i32: {e}"))),
            None => Ok(None),
        }
    }

    fn get_i64_value(&self) -> Result<Option<i64>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<i64>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse i64: {e}"))),
            None => Ok(None),
        }
    }

    fn get_f32_value(&self) -> Result<Option<f32>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<f32>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse f32: {e}"))),
            None => Ok(None),
        }
    }

    fn get_f64_value(&self) -> Result<Option<f64>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<f64>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse f64: {e}"))),
            None => Ok(None),
        }
    }

    fn get_uuid_value(&self) -> Result<Option<Uuid>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<Uuid>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse UUID: {e}"))),
            None => Ok(None),
        }
    }

    fn get_date_only_value(&self) -> Result<Option<DateOnly>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<DateOnly>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse DateOnly: {e}"))),
            None => Ok(None),
        }
    }

    fn get_time_only_value(&self) -> Result<Option<TimeOnly>, ApiError> {
        match self.current_str() {
            Some(v) => v
                .parse::<TimeOnly>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse TimeOnly: {e}"))),
            None => Ok(None),
        }
    }

    fn get_datetime_value(&self) -> Result<Option<DateTime<Utc>>, ApiError> {
        match self.current_str() {
            Some(v) => DateTime::parse_from_rfc3339(v)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(|e| ApiError::new(0, format!("Cannot parse DateTime: {e}"))),
            None => Ok(None),
        }
    }

    fn get_enum_value(
        &self,
        parser: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Option<String>, ApiError> {
        match self.current_str() {
            Some(v) => Ok(parser(v)),
            None => Ok(None),
        }
    }

    fn get_object_value(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Option<Box<dyn Parsable>>, ApiError> {
        let mut model = factory();
        let deserializers = model.get_field_deserializers();
        for (key, value) in &self.fields {
            if let Some(deserializer) = deserializers.get(key.as_str()) {
                let child = FormParseNode {
                    fields: self.fields.clone(),
                    current_value: Some(value.clone()),
                };
                deserializer(model.as_mut(), &child);
            }
        }
        Ok(Some(model))
    }

    fn get_collection_of_object_values(
        &self,
        _factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Vec<Box<dyn Parsable>>, ApiError> {
        Err(ApiError::new(
            0,
            "Form encoding does not support collections of objects".to_string(),
        ))
    }

    fn get_collection_of_primitive_values(&self) -> Result<Vec<String>, ApiError> {
        Err(ApiError::new(
            0,
            "Form encoding does not support primitive collections".to_string(),
        ))
    }

    fn get_child_node(&self, key: &str) -> Option<Box<dyn ParseNode>> {
        self.fields.get(key).map(|value| {
            Box::new(FormParseNode {
                fields: self.fields.clone(),
                current_value: Some(value.clone()),
            }) as Box<dyn ParseNode>
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_parses_form_data() {
        let node = FormParseNode::from_bytes(b"name=John&age=30").unwrap();
        assert_eq!(node.fields.get("name").unwrap(), "John");
        assert_eq!(node.fields.get("age").unwrap(), "30");
    }

    #[test]
    fn get_child_node_sets_value() {
        let node = FormParseNode::from_bytes(b"name=Alice&color=blue").unwrap();
        let child = node.get_child_node("name").unwrap();
        assert_eq!(child.get_string_value().unwrap(), Some("Alice".to_string()));
    }

    #[test]
    fn get_child_node_parses_i32() {
        let node = FormParseNode::from_bytes(b"count=42").unwrap();
        let child = node.get_child_node("count").unwrap();
        assert_eq!(child.get_i32_value().unwrap(), Some(42));
    }

    #[test]
    fn missing_key_returns_none() {
        let node = FormParseNode::from_bytes(b"name=Alice").unwrap();
        assert!(node.get_child_node("missing").is_none());
    }

    #[test]
    fn url_decodes_values() {
        let node = FormParseNode::from_bytes(b"msg=hello%20world").unwrap();
        assert_eq!(node.fields.get("msg").unwrap(), "hello world");
    }
}
