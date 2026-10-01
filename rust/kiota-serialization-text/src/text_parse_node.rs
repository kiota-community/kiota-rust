//! Text/plain [`ParseNode`] implementation — handles single string values.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::date_only::DateOnly;
use kiota_abstractions::parsable::Parsable;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::time_only::TimeOnly;

/// A [`ParseNode`] that wraps a single plain-text string value.
pub struct TextParseNode {
    value: Option<String>,
}

impl TextParseNode {
    pub fn new(value: String) -> Self {
        let trimmed = value.trim().to_string();
        Self {
            value: if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            },
        }
    }

    pub fn from_bytes(content: &[u8]) -> Result<Self, ApiError> {
        let s = std::str::from_utf8(content)
            .map_err(|e| ApiError::new(0, format!("Invalid UTF-8: {e}")))?;
        Ok(Self::new(s.to_string()))
    }
}

impl ParseNode for TextParseNode {
    fn get_string_value(&self) -> Result<Option<String>, ApiError> {
        Ok(self.value.clone())
    }

    fn get_bool_value(&self) -> Result<Option<bool>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<bool>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse bool: {e}"))),
            None => Ok(None),
        }
    }

    fn get_i32_value(&self) -> Result<Option<i32>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<i32>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse i32: {e}"))),
            None => Ok(None),
        }
    }

    fn get_i64_value(&self) -> Result<Option<i64>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<i64>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse i64: {e}"))),
            None => Ok(None),
        }
    }

    fn get_f32_value(&self) -> Result<Option<f32>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<f32>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse f32: {e}"))),
            None => Ok(None),
        }
    }

    fn get_f64_value(&self) -> Result<Option<f64>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<f64>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse f64: {e}"))),
            None => Ok(None),
        }
    }

    fn get_uuid_value(&self) -> Result<Option<Uuid>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<Uuid>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse UUID: {e}"))),
            None => Ok(None),
        }
    }

    fn get_date_only_value(&self) -> Result<Option<DateOnly>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<DateOnly>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse DateOnly: {e}"))),
            None => Ok(None),
        }
    }

    fn get_time_only_value(&self) -> Result<Option<TimeOnly>, ApiError> {
        match &self.value {
            Some(v) => v
                .parse::<TimeOnly>()
                .map(Some)
                .map_err(|e| ApiError::new(0, format!("Cannot parse TimeOnly: {e}"))),
            None => Ok(None),
        }
    }

    fn get_datetime_value(&self) -> Result<Option<DateTime<Utc>>, ApiError> {
        match &self.value {
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
        match &self.value {
            Some(v) => Ok(parser(v)),
            None => Ok(None),
        }
    }

    fn get_object_value(
        &self,
        _factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Option<Box<dyn Parsable>>, ApiError> {
        Err(ApiError::new(
            0,
            "Text/plain does not support object deserialization".to_string(),
        ))
    }

    fn get_collection_of_object_values(
        &self,
        _factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Vec<Box<dyn Parsable>>, ApiError> {
        Err(ApiError::new(
            0,
            "Text/plain does not support collections".to_string(),
        ))
    }

    fn get_collection_of_primitive_values(&self) -> Result<Vec<String>, ApiError> {
        Err(ApiError::new(
            0,
            "Text/plain does not support primitive collections".to_string(),
        ))
    }

    fn get_child_node(&self, _key: &str) -> Option<Box<dyn ParseNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string() {
        let node = TextParseNode::new("hello world".to_string());
        assert_eq!(
            node.get_string_value().unwrap(),
            Some("hello world".to_string())
        );
    }

    #[test]
    fn parse_i32() {
        let node = TextParseNode::new("42".to_string());
        assert_eq!(node.get_i32_value().unwrap(), Some(42));
    }

    #[test]
    fn parse_bool() {
        let node = TextParseNode::new("true".to_string());
        assert_eq!(node.get_bool_value().unwrap(), Some(true));
    }

    #[test]
    fn parse_uuid() {
        let id = "550e8400-e29b-41d4-a716-446655440000";
        let node = TextParseNode::new(id.to_string());
        assert_eq!(
            node.get_uuid_value().unwrap(),
            Some(id.parse::<Uuid>().unwrap())
        );
    }

    #[test]
    fn parse_empty_returns_none() {
        let node = TextParseNode::new("  ".to_string());
        assert_eq!(node.get_string_value().unwrap(), None);
    }

    #[test]
    fn from_bytes_works() {
        let node = TextParseNode::from_bytes(b"hello").unwrap();
        assert_eq!(node.get_string_value().unwrap(), Some("hello".to_string()));
    }

    #[test]
    fn object_value_returns_error() {
        let node = TextParseNode::new("test".to_string());
        assert!(node.get_object_value(&|| unreachable!()).is_err());
    }
}
