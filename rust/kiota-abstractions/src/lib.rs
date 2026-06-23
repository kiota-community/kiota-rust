//! # Kiota Abstractions for Rust
//!
//! Core abstractions for [Kiota](https://github.com/microsoft/kiota)-generated
//! Rust API clients. This crate defines the traits and types that generated code
//! depends on — it contains no generated code itself.
//!
//! ## Crate Contents
//!
//! - **Serialization traits**: [`Parsable`], [`ParseNode`], [`SerializationWriter`]
//! - **Request infrastructure**: [`RequestAdapter`], [`RequestInformation`]
//! - **Authentication**: [`AuthenticationProvider`], [`AccessTokenProvider`]
//! - **Error types**: [`ApiError`]
//! - **Date/time types**: [`DateOnly`], [`TimeOnly`]
//! - **Utilities**: [`CaseInsensitiveMap`]

pub mod api_error;
pub mod authentication;
pub mod case_insensitive_map;
pub mod date_only;
pub mod parsable;
pub mod parse_node;
pub mod parse_node_factory;
pub mod request_adapter;
pub mod request_information;
pub mod serialization_writer;
pub mod serialization_writer_factory;
pub mod time_only;

// Re-exports for convenience
pub use api_error::ApiError;
pub use authentication::{
    AccessTokenProvider, AllowedHostsValidator, AnonymousAuthenticationProvider,
    AuthenticationProvider, BaseBearerTokenAuthenticationProvider,
};
pub use case_insensitive_map::CaseInsensitiveMap;
pub use date_only::DateOnly;
pub use parsable::Parsable;
pub use parse_node::ParseNode;
pub use parse_node_factory::{ParseNodeFactory, ParseNodeFactoryRegistry};
pub use request_adapter::RequestAdapter;
pub use request_information::{HttpMethod, RequestInformation};
pub use serialization_writer::SerializationWriter;
pub use serialization_writer_factory::{SerializationWriterFactory, SerializationWriterFactoryRegistry};
pub use time_only::TimeOnly;
