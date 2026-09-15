//! # Kiota HTTP Client (reqwest)
//!
//! A [`RequestAdapter`] implementation backed by [`reqwest`] for making HTTP
//! requests in Kiota-generated Rust clients.

pub mod http_client_request_adapter;
pub mod middleware;

pub use http_client_request_adapter::HttpClientRequestAdapter;
