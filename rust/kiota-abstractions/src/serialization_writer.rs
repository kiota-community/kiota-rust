//! Trait for writing serialized representations of model objects.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::api_error::ApiError;
use crate::date_only::DateOnly;
use crate::parsable::Parsable;
use crate::time_only::TimeOnly;

/// A writer that serializes model properties into a target format (JSON, etc.).
///
/// Implementations produce byte output via [`get_serialized_content`](SerializationWriter::get_serialized_content).
/// Each `write_*` method serializes a single key-value pair.
pub trait SerializationWriter: Send + Sync {
    /// Writes a string property.
    fn write_string_value(&mut self, key: &str, value: &str) -> Result<(), ApiError>;

    /// Writes a boolean property.
    fn write_bool_value(&mut self, key: &str, value: bool) -> Result<(), ApiError>;

    /// Writes a 32-bit integer property.
    fn write_i32_value(&mut self, key: &str, value: i32) -> Result<(), ApiError>;

    /// Writes a 64-bit integer property.
    fn write_i64_value(&mut self, key: &str, value: i64) -> Result<(), ApiError>;

    /// Writes a 32-bit floating-point property.
    fn write_f32_value(&mut self, key: &str, value: f32) -> Result<(), ApiError>;

    /// Writes a 64-bit floating-point property.
    fn write_f64_value(&mut self, key: &str, value: f64) -> Result<(), ApiError>;

    /// Writes a UUID property.
    fn write_uuid_value(&mut self, key: &str, value: &Uuid) -> Result<(), ApiError>;

    /// Writes a date-only property.
    fn write_date_only_value(&mut self, key: &str, value: &DateOnly) -> Result<(), ApiError>;

    /// Writes a time-only property.
    fn write_time_only_value(&mut self, key: &str, value: &TimeOnly) -> Result<(), ApiError>;

    /// Writes a date-time property.
    fn write_datetime_value(&mut self, key: &str, value: &DateTime<Utc>) -> Result<(), ApiError>;

    /// Writes a nested object property.
    fn write_object_value(&mut self, key: &str, value: &dyn Parsable) -> Result<(), ApiError>;

    /// Writes a collection of nested objects.
    fn write_collection_of_object_values(
        &mut self,
        key: &str,
        values: &[&dyn Parsable],
    ) -> Result<(), ApiError>;

    /// Writes additional untyped data (e.g. extra JSON properties).
    ///
    /// Keys are property names; values are `serde_json::Value` instances.
    fn write_additional_data(&mut self, data: &HashMap<String, Value>) -> Result<(), ApiError>;

    /// Returns the serialized content as a byte vector.
    fn get_serialized_content(&self) -> Result<Vec<u8>, ApiError>;
}
