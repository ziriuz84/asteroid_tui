// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Input validators for scheduling and observatory forms.

use chrono::NaiveDate;

/// Validates if a date is valid (checks if the date actually exists).
pub fn validate_date(year: u32, month: u32, day: u32) -> bool {
    NaiveDate::from_ymd_opt(year as i32, month, day).is_some()
}

/// Validates year input (1900-2200).
pub fn validate_year(year: &str) -> bool {
    year.parse::<u32>()
        .map(|y| (1900..=2200).contains(&y))
        .unwrap_or(false)
}

/// Validates month input (1-12).
pub fn validate_month(month: &str) -> bool {
    month.parse::<u32>()
        .map(|m| (1..=12).contains(&m))
        .unwrap_or(false)
}

/// Validates day input (1-31).
pub fn validate_day(day: &str) -> bool {
    day.parse::<u32>()
        .map(|d| (1..=31).contains(&d))
        .unwrap_or(false)
}

/// Validates hour input (0-23).
pub fn validate_hour(hour: &str) -> bool {
    hour.parse::<u32>().map(|h| h <= 23).unwrap_or(false)
}

/// Validates minute input (0-59).
pub fn validate_minute(minute: &str) -> bool {
    minute.parse::<u32>().map(|m| m <= 59).unwrap_or(false)
}

/// Validates a positive integer string.
pub fn validate_positive_integer(value: &str) -> bool {
    value.parse::<u32>().is_ok()
}

/// Validates latitude value (-90 to 90).
pub fn validate_latitude(lat: &str) -> bool {
    lat.parse::<f32>()
        .map(|l| (-90.0..=90.0).contains(&l))
        .unwrap_or(false)
}

/// Validates longitude value (-180 to 180).
pub fn validate_longitude(lon: &str) -> bool {
    lon.parse::<f32>()
        .map(|l| (-180.0..=180.0).contains(&l))
        .unwrap_or(false)
}

/// Maps object type string to MPC code.
pub fn map_object_type_to_code(object_type: &str) -> &str {
    match object_type {
        "Asteroid" => "mp",
        "NEO" => "neo",
        "Comet" => "cmt",
        _ => "mp",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date() {
        assert!(validate_date(2000, 1, 1));
        assert!(validate_date(2000, 2, 29));
        assert!(validate_date(2024, 12, 31));
        assert!(!validate_date(2000, 2, 30));
        assert!(!validate_date(2001, 2, 29));
        assert!(!validate_date(2000, 13, 1));
        assert!(!validate_date(2000, 0, 1));
        assert!(!validate_date(2000, 1, 0));
        assert!(!validate_date(2000, 1, 32));
    }

    #[test]
    fn test_validate_year() {
        assert!(validate_year("2000"));
        assert!(validate_year("2024"));
        assert!(validate_year("1900"));
        assert!(validate_year("2200"));
        assert!(!validate_year("1899"));
        assert!(!validate_year("2201"));
        assert!(!validate_year("abc"));
        assert!(!validate_year(""));
    }

    #[test]
    fn test_validate_month() {
        for i in 1..=12 {
            assert!(validate_month(&i.to_string()));
        }
        assert!(!validate_month("0"));
        assert!(!validate_month("13"));
        assert!(!validate_month("abc"));
        assert!(!validate_month(""));
    }

    #[test]
    fn test_validate_day() {
        for i in 1..=31 {
            assert!(validate_day(&i.to_string()));
        }
        assert!(!validate_day("0"));
        assert!(!validate_day("32"));
        assert!(!validate_day("abc"));
        assert!(!validate_day(""));
    }

    #[test]
    fn test_validate_hour() {
        for i in 0..=23 {
            assert!(validate_hour(&i.to_string()));
        }
        assert!(!validate_hour("24"));
        assert!(!validate_hour("abc"));
        assert!(!validate_hour(""));
    }

    #[test]
    fn test_validate_minute() {
        for i in 0..=59 {
            assert!(validate_minute(&i.to_string()));
        }
        assert!(!validate_minute("60"));
        assert!(!validate_minute("abc"));
        assert!(!validate_minute(""));
    }

    #[test]
    fn test_map_object_type_to_code() {
        assert_eq!(map_object_type_to_code("Asteroid"), "mp");
        assert_eq!(map_object_type_to_code("NEO"), "neo");
        assert_eq!(map_object_type_to_code("Comet"), "cmt");
        assert_eq!(map_object_type_to_code("Unknown"), "mp");
    }

    #[test]
    fn test_validate_latitude() {
        assert!(validate_latitude("0"));
        assert!(validate_latitude("45.5"));
        assert!(validate_latitude("-45.5"));
        assert!(validate_latitude("90"));
        assert!(validate_latitude("-90"));
        assert!(!validate_latitude("91"));
        assert!(!validate_latitude("-91"));
        assert!(!validate_latitude("abc"));
        assert!(!validate_latitude(""));
    }

    #[test]
    fn test_validate_longitude() {
        assert!(validate_longitude("0"));
        assert!(validate_longitude("120.5"));
        assert!(validate_longitude("-120.5"));
        assert!(validate_longitude("180"));
        assert!(validate_longitude("-180"));
        assert!(!validate_longitude("181"));
        assert!(!validate_longitude("-181"));
        assert!(!validate_longitude("abc"));
        assert!(!validate_longitude(""));
    }
}
