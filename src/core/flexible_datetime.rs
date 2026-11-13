//! Tolerant date-time (de)serialization.
//!
//! chrono's stock `DateTime` serde impl only accepts strict RFC 3339. Real
//! APIs also emit timestamps with a space separator, without an offset, or
//! as a bare `YYYY-MM-DD`. This module accepts all of those on the way in
//! and always writes strict RFC 3339 on the way out, so round-tripping a
//! value normalizes it.
//!
//! Values keep the offset they arrived with (`DateTime<FixedOffset>`) rather
//! than being silently shifted to UTC. Call `.with_timezone(&Utc)` when you
//! want a UTC instant.

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Formats accepted for a naive (offset-less) timestamp. Tried in order.
const NAIVE_FORMATS: &[&str] = &[
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%d %H:%M:%S",
];

/// Parses the tolerated timestamp shapes, in cheapest-first order.
///
/// An offset-less timestamp is interpreted as UTC — the same assumption
/// chrono makes for `NaiveDateTime::and_utc`, and the only defensible one
/// without out-of-band knowledge of the server timezone.
pub fn parse(value: &str) -> Result<DateTime<FixedOffset>, String> {
    let trimmed = value.trim();

    if let Ok(parsed) = DateTime::parse_from_rfc3339(trimmed) {
        return Ok(parsed);
    }

    for format in NAIVE_FORMATS {
        if let Ok(naive) = NaiveDateTime::parse_from_str(trimmed, format) {
            return Ok(Utc.from_utc_datetime(&naive).fixed_offset());
        }
    }

    // Bare date: anchor at midnight UTC.
    if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        if let Some(naive) = date.and_hms_opt(0, 0, 0) {
            return Ok(Utc.from_utc_datetime(&naive).fixed_offset());
        }
    }

    Err(format!("`{}` is not a recognized date-time", trimmed))
}

/// `#[serde(with = "...::offset")]` for a required `DateTime<FixedOffset>`.
pub mod offset {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &DateTime<FixedOffset>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.to_rfc3339().serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<FixedOffset>, D::Error> {
        let raw = String::deserialize(deserializer)?;
        super::parse(&raw).map_err(serde::de::Error::custom)
    }

    /// `#[serde(with = "...::offset::option")]` for `Option<DateTime<FixedOffset>>`.
    ///
    /// A JSON `null` decodes to `None` rather than erroring, which keeps an
    /// explicitly-null optional timestamp from failing the whole response.
    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer>(
            value: &Option<DateTime<FixedOffset>>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(inner) => inner.to_rfc3339().serialize(serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<DateTime<FixedOffset>>, D::Error> {
            let raw = Option::<String>::deserialize(deserializer)?;
            match raw {
                Some(text) => super::super::parse(&text)
                    .map(Some)
                    .map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}

/// `#[serde(with = "...::date")]` for a required `NaiveDate`.
///
/// Accepts a full timestamp as well as a bare date, discarding the time
/// component — some APIs return `2024-01-05T00:00:00Z` for a `date` field.
pub mod date {
    use super::*;

    pub fn serialize<S: Serializer>(value: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
        value.format("%Y-%m-%d").to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
        let raw = String::deserialize(deserializer)?;
        let trimmed = raw.trim();
        if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
            return Ok(date);
        }
        super::parse(trimmed)
            .map(|value| value.date_naive())
            .map_err(serde::de::Error::custom)
    }

    /// `#[serde(with = "...::date::option")]` for `Option<NaiveDate>`.
    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer>(
            value: &Option<NaiveDate>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(inner) => inner.format("%Y-%m-%d").to_string().serialize(serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<NaiveDate>, D::Error> {
            let raw = Option::<String>::deserialize(deserializer)?;
            let Some(text) = raw else { return Ok(None) };
            let trimmed = text.trim();
            if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
                return Ok(Some(date));
            }
            super::super::parse(trimmed)
                .map(|value| Some(value.date_naive()))
                .map_err(serde::de::Error::custom)
        }
    }
}
