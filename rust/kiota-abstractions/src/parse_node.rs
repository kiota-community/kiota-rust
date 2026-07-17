//! Trait for reading typed values from a deserialized node tree.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api_error::ApiError;
use crate::date_only::DateOnly;
use crate::parsable::Parsable;
use crate::time_only::TimeOnly;

/// A node in a deserialized tree that provides typed accessors for its value.
///
/// Implementations back onto a concrete format (JSON, XML, etc.) and convert
/// raw tokens into Rust types.
///
/// This trait is dyn-compatible: methods that would require generics instead
/// accept factory closures that produce `Box<dyn Parsable>`.
pub trait ParseNode: Send + Sync {
    /// Reads the node value as a `String`.
    fn get_string_value(&self) -> Result<Option<String>, ApiError>;

    /// Reads the node value as a `bool`.
    fn get_bool_value(&self) -> Result<Option<bool>, ApiError>;

    /// Reads the node value as an `i32`.
    fn get_i32_value(&self) -> Result<Option<i32>, ApiError>;

    /// Reads the node value as an `i64`.
    fn get_i64_value(&self) -> Result<Option<i64>, ApiError>;

    /// Reads the node value as an `f32`.
    fn get_f32_value(&self) -> Result<Option<f32>, ApiError>;

    /// Reads the node value as an `f64`.
    fn get_f64_value(&self) -> Result<Option<f64>, ApiError>;

    /// Reads the node value as a [`Uuid`].
    fn get_uuid_value(&self) -> Result<Option<Uuid>, ApiError>;

    /// Reads the node value as a [`DateOnly`].
    fn get_date_only_value(&self) -> Result<Option<DateOnly>, ApiError>;

    /// Reads the node value as a [`TimeOnly`].
    fn get_time_only_value(&self) -> Result<Option<TimeOnly>, ApiError>;

    /// Reads the node value as a [`DateTime<Utc>`].
    fn get_datetime_value(&self) -> Result<Option<DateTime<Utc>>, ApiError>;

    /// Reads the node value as an enum variant.
    ///
    /// The `parser` function converts the raw string representation into a
    /// boxed string that the caller can then map to the desired enum type.
    fn get_enum_value(
        &self,
        parser: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Option<String>, ApiError>;

    /// Reads the node value as a structured object.
    ///
    /// The `factory` function creates a default [`Parsable`] whose fields are
    /// then populated via its field deserializers.
    fn get_object_value(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Option<Box<dyn Parsable>>, ApiError>;

    /// Reads the node value as a collection of structured objects.
    fn get_collection_of_object_values(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Vec<Box<dyn Parsable>>, ApiError>;

    /// Reads the node value as a collection of primitive strings.
    fn get_collection_of_primitive_values(&self) -> Result<Vec<String>, ApiError>;

    /// Returns a child node for the given key, or `None` if the key is absent.
    fn get_child_node(&self, key: &str) -> Option<Box<dyn ParseNode>>;
}
