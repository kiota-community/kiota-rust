//! Factory for creating [`JsonParseNode`] instances.

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::parse_node_factory::ParseNodeFactory;

use crate::json_parse_node::JsonParseNode;

/// Factory that creates [`JsonParseNode`] instances from `application/json` content.
pub struct JsonParseNodeFactory;

impl ParseNodeFactory for JsonParseNodeFactory {
    fn valid_content_type(&self) -> &str {
        "application/json"
    }

    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, ApiError> {
        if content_type != self.valid_content_type() {
            return Err(ApiError::new(
                0,
                format!(
                    "expected '{}', got '{content_type}'",
                    self.valid_content_type()
                ),
            ));
        }
        let node = JsonParseNode::from_bytes(content)?;
        Ok(Box::new(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parse_node_for_json() {
        let factory = JsonParseNodeFactory;
        let content = b"{\"key\": \"value\"}";
        let node = factory
            .get_root_parse_node("application/json", content)
            .unwrap();
        let child = node.get_child_node("key").unwrap();
        assert_eq!(child.get_string_value().unwrap(), Some("value".to_string()));
    }

    #[test]
    fn rejects_wrong_content_type() {
        let factory = JsonParseNodeFactory;
        let result = factory.get_root_parse_node("text/plain", b"{}");
        assert!(result.is_err());
    }

    #[test]
    fn valid_content_type_is_json() {
        let factory = JsonParseNodeFactory;
        assert_eq!(factory.valid_content_type(), "application/json");
    }
}
