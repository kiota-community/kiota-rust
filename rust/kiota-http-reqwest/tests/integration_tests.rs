//! Integration tests for the HTTP client request adapter using wiremock.

use std::collections::HashMap;
use std::sync::Arc;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use kiota_abstractions::api_error::ApiError;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::parsable::{FieldDeserializer, Parsable};
use kiota_abstractions::parse_node_factory::ParseNodeFactoryRegistry;
use kiota_abstractions::request_adapter::RequestAdapter;
use kiota_abstractions::request_information::{HttpMethod, RequestInformation};
use kiota_abstractions::serialization_writer::SerializationWriter;
use kiota_abstractions::serialization_writer_factory::SerializationWriterFactoryRegistry;

use kiota_http_reqwest::http_client_request_adapter::HttpClientRequestAdapter;
use kiota_serialization_json::json_parse_node_factory::JsonParseNodeFactory;
use kiota_serialization_json::json_serialization_writer_factory::JsonSerializationWriterFactory;

fn create_adapter(base_url: &str) -> HttpClientRequestAdapter {
    let auth = Arc::new(AnonymousAuthenticationProvider);

    let mut parse_registry = ParseNodeFactoryRegistry::new();
    parse_registry.register(Arc::new(JsonParseNodeFactory));

    let mut writer_registry = SerializationWriterFactoryRegistry::new();
    writer_registry.register(Arc::new(JsonSerializationWriterFactory));

    let mut adapter =
        HttpClientRequestAdapter::new(auth, Arc::new(parse_registry), Arc::new(writer_registry));
    adapter.set_base_url(base_url);
    adapter
}

fn make_get_request(uri: &str) -> RequestInformation {
    let mut info = RequestInformation::new();
    info.http_method = HttpMethod::Get;
    info.set_uri(uri);
    info
}

fn make_post_request(uri: &str) -> RequestInformation {
    let mut info = RequestInformation::new();
    info.http_method = HttpMethod::Post;
    info.set_uri(uri);
    info
}

// --- Test model ---

#[derive(Default, Debug)]
struct TestModel {
    name: Option<String>,
    age: Option<i32>,
}

impl Parsable for TestModel {
    fn get_field_deserializers(&self) -> HashMap<String, FieldDeserializer> {
        HashMap::new()
    }

    fn serialize(&self, writer: &mut dyn SerializationWriter) -> Result<(), ApiError> {
        if let Some(ref name) = self.name {
            writer.write_string_value("name", name)?;
        }
        if let Some(age) = self.age {
            writer.write_i32_value("age", age)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// --- Tests ---

#[tokio::test]
async fn test_send_no_content_succeeds_on_204() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/items"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let info = make_post_request(&format!("{}/api/items", server.uri()));

    let result = adapter.send_no_content(&info, None).await;
    assert!(result.is_ok(), "send_no_content should succeed on 204");
}

#[tokio::test]
async fn test_send_returns_error_on_404() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let info = make_get_request(&format!("{}/api/missing", server.uri()));

    let result = adapter.send::<TestModel>(&info, None).await;
    assert!(result.is_err(), "Should return error on 404");

    let err = result.unwrap_err();
    assert_eq!(err.status_code, 404);
    assert_eq!(err.response_body, Some("Not Found".to_string()));
}

#[tokio::test]
async fn test_send_returns_error_on_500() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/error"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let info = make_get_request(&format!("{}/api/error", server.uri()));

    let result = adapter.send::<TestModel>(&info, None).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.status_code, 500);
}

#[tokio::test]
async fn test_send_get_returns_none_for_empty_body() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/empty"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let info = make_get_request(&format!("{}/api/empty", server.uri()));

    let result = adapter.send::<TestModel>(&info, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none(), "Empty body should return None");
}

#[tokio::test]
async fn test_send_no_content_fails_on_error_status() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/items/1"))
        .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let mut info = RequestInformation::new();
    info.http_method = HttpMethod::Delete;
    info.set_uri(&format!("{}/api/items/1", server.uri()));

    let result = adapter.send_no_content(&info, None).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().status_code, 403);
}

#[tokio::test]
async fn test_get_base_url_and_set_base_url() {
    let adapter = create_adapter("https://api.example.com");
    assert_eq!(adapter.get_base_url(), "https://api.example.com");
}

#[tokio::test]
async fn test_request_hits_correct_endpoint() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"name": "Alice"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let adapter = create_adapter(&server.uri());
    let info = make_get_request(&format!("{}/api/v2/users", server.uri()));

    let _ = adapter.send::<TestModel>(&info, None).await;
    // wiremock will panic on drop if the mock wasn't hit exactly once
}
