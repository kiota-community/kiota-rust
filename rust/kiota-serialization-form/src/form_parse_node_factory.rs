//! Factory for [`FormParseNode`].

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::parse_node::ParseNode;
use kiota_abstractions::parse_node_factory::ParseNodeFactory;

use crate::form_parse_node::FormParseNode;

const CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

pub struct FormParseNodeFactory;

impl ParseNodeFactory for FormParseNodeFactory {
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
        let node = FormParseNode::from_bytes(content)?;
        Ok(Box::new(node))
    }
}
