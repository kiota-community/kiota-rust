//! Factory for [`TextParseNode`].

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::parse_node_factory::ParseNodeFactory;

use crate::text_parse_node::TextParseNode;

const CONTENT_TYPE: &str = "text/plain";

pub struct TextParseNodeFactory;

impl ParseNodeFactory for TextParseNodeFactory {
    fn valid_content_type(&self) -> &str {
        CONTENT_TYPE
    }

    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, ApiError> {
        if content_type != CONTENT_TYPE {
            return Err(ApiError::new(
                0,
                format!("Expected {CONTENT_TYPE}, got {content_type}"),
            ));
        }
        let node = TextParseNode::from_bytes(content)?;
        Ok(Box::new(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_content_type_is_text() {
        let factory = TextParseNodeFactory;
        assert_eq!(factory.valid_content_type(), "text/plain");
    }

    #[test]
    fn creates_parse_node_for_text() {
        let factory = TextParseNodeFactory;
        let node = factory.get_root_parse_node("text/plain", b"hello");
        assert!(node.is_ok());
    }

    #[test]
    fn rejects_wrong_content_type() {
        let factory = TextParseNodeFactory;
        let node = factory.get_root_parse_node("application/json", b"hello");
        assert!(node.is_err());
    }
}
