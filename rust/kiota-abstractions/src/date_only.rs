//! A date-only type (no time component) for API serialization.

use std::fmt;
use std::str::FromStr;

use crate::api_error::ApiError;

/// Represents a calendar date without a time-of-day or timezone component.
///
/// Serialized as `YYYY-MM-DD` (ISO 8601 date format).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateOnly {
    /// Year component.
    pub year: i32,
    /// Month component (1–12).
    pub month: u32,
    /// Day component (1–31).
    pub day: u32,
}

impl DateOnly {
    /// Creates a new `DateOnly`.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

impl fmt::Display for DateOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for DateOnly {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(ApiError::new(
                0,
                format!("invalid date format, expected YYYY-MM-DD: {s}"),
            ));
        }

        let year = parts[0].parse::<i32>().map_err(|e| {
            ApiError::new(0, format!("invalid year in date '{s}': {e}"))
        })?;
        let month = parts[1].parse::<u32>().map_err(|e| {
            ApiError::new(0, format!("invalid month in date '{s}': {e}"))
        })?;
        let day = parts[2].parse::<u32>().map_err(|e| {
            ApiError::new(0, format!("invalid day in date '{s}': {e}"))
        })?;

        Ok(Self { year, month, day })
    }
}

impl From<chrono::NaiveDate> for DateOnly {
    fn from(d: chrono::NaiveDate) -> Self {
        use chrono::Datelike;
        Self {
            year: d.year(),
            month: d.month(),
            day: d.day(),
        }
    }
}

impl From<DateOnly> for chrono::NaiveDate {
    fn from(d: DateOnly) -> Self {
        chrono::NaiveDate::from_ymd_opt(d.year, d.month, d.day)
            .expect("DateOnly contained an invalid calendar date")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_pads_correctly() {
        assert_eq!(DateOnly::new(2024, 1, 5).to_string(), "2024-01-05");
        assert_eq!(DateOnly::new(999, 12, 31).to_string(), "0999-12-31");
    }

    #[test]
    fn from_str_valid() {
        let d: DateOnly = "2024-06-15".parse().unwrap();
        assert_eq!(d, DateOnly::new(2024, 6, 15));
    }

    #[test]
    fn from_str_invalid_format() {
        assert!("2024/06/15".parse::<DateOnly>().is_err());
        assert!("not-a-date".parse::<DateOnly>().is_err());
    }

    #[test]
    fn roundtrip_chrono() {
        let original = DateOnly::new(2024, 3, 14);
        let naive: chrono::NaiveDate = original.into();
        let back: DateOnly = naive.into();
        assert_eq!(original, back);
    }

    #[test]
    fn display_roundtrip() {
        let d = DateOnly::new(2024, 11, 2);
        let s = d.to_string();
        let parsed: DateOnly = s.parse().unwrap();
        assert_eq!(d, parsed);
    }
}
