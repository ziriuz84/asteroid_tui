//! # Utils
//!
//! Library for general astronomical utility functions
//!
//! This library provides utility functions for coordinate conversions, angle transformations,
//! and visibility calculations for astronomical objects.
//!
//! Main functionalities include:
//! - Converting between different coordinate formats (hour angle, declination, degrees, radians)
//! - Checking if an object is visible from a given location at a specific time
//!
//! Example usage for coordinate conversion:
//!
//! ```rust
//! use asteroid_tui::utils;
//!
//! // Convert hour angle to radians
//! let ra_rad = utils::convert_hour_angle_to_radians("12:34:56");
//!
//! // Convert declination to degrees
//! let dec_deg = utils::convert_dec_to_deg("+45:30:15");
//! ```
//!
//! Example usage for visibility check:
//!
//! ```rust
//! use asteroid_tui::utils;
//! use chrono::{TimeZone, Utc};
//!
//! let date = Utc.with_ymd_and_hms(2024, 3, 27, 20, 0, 0).unwrap();
//! let is_visible = utils::is_visible("12:34:56", "+45:30:15", date);
//! if is_visible {
//!     println!("Object is visible!");
//! }
//! ```

use crate::settings::Settings;
//use astro;
use astronav::{
    coords::{dms_to_deg, hms_to_deg, star::AltAzBuilder},
    time::{gmst_in_degrees, julian_day_number, julian_time, lmst_in_degrees},
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::f64::consts::PI;

fn convert_angle(input: &str, factor_deg: f64, factor_min: f64, factor_sec: f64) -> f64 {
    let parts: Vec<&str> = input.split(|c| c == ':' || c == ' ').collect();
    let deg: f64 = parts[0].parse().unwrap();
    let min: f64 = parts[1].parse().unwrap();
    let sec: f64 = parts[2].parse().unwrap();
    deg * factor_deg + min * factor_min + sec * factor_sec
}

/// Converts right ascension from hour angle format to radians
///
/// Right ascension is typically expressed in hours, minutes, and seconds (HMS format),
/// where 24 hours = 360 degrees. This function converts it to radians for use in
/// astronomical calculations.
///
/// # Arguments
///
/// * `ra`: Right ascension in hour angle format as "hh:mm:ss" or "hh mm ss"
///   (e.g., "12:34:56" or "12 34 56")
///
/// # Returns
///
/// The right ascension in radians (0 to 2π)
///
/// # Panics
///
/// This function will panic if:
/// - The input string cannot be split into exactly 3 parts
/// - Any part cannot be parsed as a number
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
///
/// let ra_rad = utils::convert_hour_angle_to_radians("12:34:56");
/// println!("RA in radians: {:.6}", ra_rad);
/// ```
pub fn convert_hour_angle_to_radians(ra: &str) -> f64 {
    // hour angle in radians: (h, m, s)*15 => degrees then convert to radians.
    convert_angle(ra, 15.0, 15.0 / 60.0, 15.0 / 3600.0) * (PI / 180.0)
}

/// Converts declination from degrees, minutes, seconds format to radians
///
/// Declination is expressed in degrees, minutes, and seconds (DMS format).
/// This function converts it to radians for use in astronomical calculations.
///
/// # Arguments
///
/// * `dec`: Declination in DMS format as "±dd:mm:ss" or "±dd mm ss"
///   (e.g., "+45:30:15" or "-12 34 56")
///
/// # Returns
///
/// The declination in radians (-π/2 to π/2)
///
/// # Panics
///
/// This function will panic if:
/// - The input string cannot be split into exactly 3 parts
/// - Any part cannot be parsed as a number
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
///
/// let dec_rad = utils::convert_dec_to_radians("+45:30:15");
/// println!("Dec in radians: {:.6}", dec_rad);
/// ```
pub fn convert_dec_to_radians(dec: &str) -> f64 {
    convert_angle(dec, 1.0, 1.0 / 60.0, 1.0 / 3600.0) * (PI / 180.0)
}

/// Converts right ascension from hour angle format to decimal degrees
///
/// Converts right ascension from HMS format (hours, minutes, seconds) to decimal degrees.
/// Since 24 hours = 360 degrees, each hour equals 15 degrees.
///
/// # Arguments
///
/// * `ra`: Right ascension in hour angle format as "hh:mm:ss" or "hh mm ss"
///   (e.g., "12:34:56")
///
/// # Returns
///
/// The right ascension in decimal degrees (0 to 360)
///
/// # Panics
///
/// This function will panic if:
/// - The input string cannot be split into exactly 3 parts
/// - Any part cannot be parsed as a number
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
///
/// let ra_deg = utils::convert_hour_angle_to_dec("12:34:56");
/// println!("RA in degrees: {:.6}", ra_deg); // Approximately 188.733 degrees
/// ```
pub fn convert_hour_angle_to_dec(ra: &str) -> f64 {
    convert_angle(ra, 15.0, 15.0 / 60.0, 15.0 / 3600.0)
}

/// Converts declination from degrees, minutes, seconds format to decimal degrees
///
/// Converts declination from DMS format to decimal degrees for easier calculations.
///
/// # Arguments
///
/// * `dec`: Declination in DMS format as "±dd:mm:ss" or "±dd mm ss"
///   (e.g., "+45:30:15" or "-12 34 56")
///
/// # Returns
///
/// The declination in decimal degrees (-90 to +90)
///
/// # Panics
///
/// This function will panic if:
/// - The input string cannot be split into exactly 3 parts
/// - Any part cannot be parsed as a number
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
///
/// let dec_deg = utils::convert_dec_to_deg("+45:30:15");
/// println!("Dec in degrees: {:.6}", dec_deg); // Approximately 45.504 degrees
/// ```
pub fn convert_dec_to_deg(dec: &str) -> f64 {
    convert_angle(dec, 1.0, 1.0 / 60.0, 1.0 / 3600.0)
}

/// Converts an angle from degrees to radians
///
/// Simple conversion using the standard formula: radians = degrees × π / 180
///
/// # Arguments
///
/// * `deg`: Angle in degrees
///
/// # Returns
///
/// The angle in radians
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
///
/// let rad = utils::convert_deg_to_radians(180.0);
/// println!("180 degrees = {} radians (π)", rad); // Approximately 3.14159
/// ```
pub fn convert_deg_to_radians(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

/// Checks if a celestial object is visible from the observatory location at a given time
///
/// This function calculates the object's altitude and azimuth at the specified time and
/// checks if it meets the visibility criteria defined in the settings. The object must be
/// above the minimum altitude thresholds for the appropriate direction (north, south, east, west).
///
/// Visibility is determined by:
/// - Altitude above the horizon (must exceed direction-specific minimums)
/// - Azimuth direction (different altitude requirements for different compass directions)
///
/// # Arguments
///
/// * `ra`: Right ascension in hour angle format as "hh:mm:ss" or "hh mm ss"
/// * `dec`: Declination in DMS format as "±dd:mm:ss" or "±dd mm ss"
/// * `date`: Date and time of observation in UTC
///
/// # Returns
///
/// `true` if the object is visible (meets all altitude requirements), `false` otherwise
///
/// # Panics
///
/// This function will panic if:
/// - Settings cannot be loaded
/// - Coordinate conversion fails
/// - Astronomical calculations fail
///
/// # Example
///
/// ```rust
/// use asteroid_tui::utils;
/// use chrono::{TimeZone, Utc};
///
/// let date = Utc.with_ymd_and_hms(2024, 3, 27, 20, 0, 0).unwrap();
/// let is_visible = utils::is_visible("12:34:56", "+45:30:15", date);
///
/// if is_visible {
///     println!("Object is visible at the specified time!");
/// } else {
///     println!("Object is below the horizon or doesn't meet visibility criteria");
/// }
/// ```
pub fn is_visible(ra: &str, dec: &str, date: DateTime<Utc>) -> bool {
    let settings = Settings::new().unwrap();
    let longitude = *settings.get_longitude();
    let julian_day = julian_day_number(date.day() as u8, date.month() as u8, date.year() as u16);
    let julian_time_value = julian_time(
        julian_day,
        date.hour() as u8,
        date.minute() as u8,
        date.second() as u8,
        0.0,
    );
    let greenwich_mean = gmst_in_degrees(julian_time_value);
    let local_mean = lmst_in_degrees(greenwich_mean, longitude as f64);
    let alt = AltAzBuilder::new()
        .dec(dms_to_deg(dec).unwrap())
        .lat(*settings.get_latitude() as f64)
        .lmst(local_mean)
        .ra(hms_to_deg(ra).unwrap())
        .seal()
        .build();
    let altitude = alt.get_altitude();
    let azimuth = alt.get_azimuth();
    let conditions = [
        ((45.0, 135.0), *settings.get_south_altitude() as f64),
        ((135.0, 225.0), *settings.get_south_altitude() as f64),
        ((225.0, 315.0), *settings.get_west_altitude() as f64),
    ];

    if !(45.0..=315.0).contains(&azimuth) && altitude > *settings.get_north_altitude() as f64 {
        return true;
    }

    for &((min_az, max_az), min_alt) in conditions.iter() {
        if azimuth > min_az && azimuth < max_az && altitude > min_alt {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_visible_known_object() {
        let test_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let ra = "12:0:0";
        let dec = "0:0:0";
        let object_is_visible = is_visible(ra, dec, test_date);
        assert!(object_is_visible);
    }

    #[test]
    fn test_is_not_visible() {
        let test_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let ra = "12:0:0";
        let dec = "-80:0:0";
        let object_is_visible = is_visible(ra, dec, test_date);
        assert!(!object_is_visible);
    }
}
