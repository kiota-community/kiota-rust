//! A time-only type (no date or timezone component) for API serialization.

use std::fmt;
use std::str::FromStr;

use crate::api_error::ApiError;

/// Represents a time-of-day without a date or timezone component.
///
/// Serialized as `HH:MM:SS` (ISO 8601 time format).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeOnly {
    /// Hour component (0–23).
    pub hour: u32,
    /// Minute component (0–59).
    pub minute: u32,
    /// Second component (0–59).
    pub second: u32,
}

impl TimeOnly {
    /// Creates a new `TimeOnly`.
    pub fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for TimeOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl FromStr for TimeOnly {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(ApiError::new(
                0,
                format!("invalid time format, expected HH:MM:SS: {s}"),
            ));
        }

        let hour = parts[0].parse::<u32>().map_err(|e| {
            ApiError::new(0, format!("invalid hour in time '{s}': {e}"))
        })?;
        let minute = parts[1].parse::<u32>().map_err(|e| {
            ApiError::new(0, format!("invalid minute in time '{s}': {e}"))
        })?;
        let second = parts[2].parse::<u32>().map_err(|e| {
            ApiError::new(0, format!("invalid second in time '{s}': {e}"))
        })?;

        Ok(Self {
            hour,
            minute,
            second,
        })
    }
}

impl From<chrono::NaiveTime> for TimeOnly {
    fn from(t: chrono::NaiveTime) -> Self {
        use chrono::Timelike;
        Self {
            hour: t.hour(),
            minute: t.minute(),
            second: t.second(),
        }
    }
}

impl From<TimeOnly> for chrono::NaiveTime {
    fn from(t: TimeOnly) -> Self {
        chrono::NaiveTime::from_hms_opt(t.hour, t.minute, t.second)
            .expect("TimeOnly contained an invalid time")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_pads_correctly() {
        assert_eq!(TimeOnly::new(9, 5, 3).to_string(), "09:05:03");
        assert_eq!(TimeOnly::new(23, 59, 59).to_string(), "23:59:59");
    }

    #[test]
    fn from_str_valid() {
        let t: TimeOnly = "14:30:00".parse().unwrap();
        assert_eq!(t, TimeOnly::new(14, 30, 0));
    }

    #[test]
    fn from_str_invalid_format() {
        assert!("14-30-00".parse::<TimeOnly>().is_err());
        assert!("not-a-time".parse::<TimeOnly>().is_err());
    }

    #[test]
    fn roundtrip_chrono() {
        let original = TimeOnly::new(8, 15, 30);
        let naive: chrono::NaiveTime = original.into();
        let back: TimeOnly = naive.into();
        assert_eq!(original, back);
    }

    #[test]
    fn display_roundtrip() {
        let t = TimeOnly::new(0, 0, 0);
        let s = t.to_string();
        let parsed: TimeOnly = s.parse().unwrap();
        assert_eq!(t, parsed);
    }
}
