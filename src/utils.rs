//! # Utils
//!
//! Library for general utilities

use crate::settings::{Observatory, Settings};
//use astro;
use astronav::{
    coords::{dms_to_deg, hms_to_deg, star::AltAzBuilder},
    time::{gmst_in_degrees, julian_day_number, julian_time, lmst_in_degrees},
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::f64::consts::PI;

fn convert_angle(input: &str, factor_deg: f64, factor_min: f64, factor_sec: f64) -> f64 {
    let parts: Vec<&str> = input.split([':', ' ']).collect();
    let deg: f64 = parts[0].parse().unwrap();
    let min: f64 = parts[1].parse().unwrap();
    let sec: f64 = parts[2].parse().unwrap();
    deg * factor_deg + min * factor_min + sec * factor_sec
}

/// Function to convert hour angle to radians
///
/// * `ra`: Right ascension in hour angle formatted as "hh:mm:ss"
///
pub fn convert_hour_angle_to_radians(ra: &str) -> f64 {
    // hour angle in radians: (h, m, s)*15 => degrees then convert to radians.
    convert_angle(ra, 15.0, 15.0 / 60.0, 15.0 / 3600.0) * (PI / 180.0)
}

/// Function to convert declination to radians
///
/// * `dec`: Declination in degrees formatted as "dd:mm:ss"
pub fn convert_dec_to_radians(dec: &str) -> f64 {
    convert_angle(dec, 1.0, 1.0 / 60.0, 1.0 / 3600.0) * (PI / 180.0)
}

/// Function to convert hour angle to decimal degrees
///
/// * `ra`: Right ascension in hour angle formatted as "hh:mm:ss"
pub fn convert_hour_angle_to_dec(ra: &str) -> f64 {
    convert_angle(ra, 15.0, 15.0 / 60.0, 15.0 / 3600.0)
}

/// Function to convert declination to decimal degrees
///
/// * `dec`: Declination in degrees formatted as "dd:mm:ss"
pub fn convert_dec_to_deg(dec: &str) -> f64 {
    convert_angle(dec, 1.0, 1.0 / 60.0, 1.0 / 3600.0)
}

/// Function to convert degrees to radians
///
/// * `deg`: Angle in degrees
pub fn convert_deg_to_radians(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

/// Checks visibility using explicit observatory coordinates and horizon limits.
pub fn is_visible_with_observatory(
    ra: &str,
    dec: &str,
    date: DateTime<Utc>,
    observatory: &Observatory,
) -> bool {
    let longitude = observatory.longitude;
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
        .lat(observatory.latitude as f64)
        .lmst(local_mean)
        .ra(hms_to_deg(ra).unwrap())
        .seal()
        .build();
    let altitude = alt.get_altitude();
    let azimuth = alt.get_azimuth();
    let conditions = [
        ((45.0, 135.0), observatory.south_altitude as f64),
        ((135.0, 225.0), observatory.south_altitude as f64),
        ((225.0, 315.0), observatory.west_altitude as f64),
    ];

    if !(45.0..=315.0).contains(&azimuth) && altitude > observatory.north_altitude as f64 {
        return true;
    }

    for &((min_az, max_az), min_alt) in conditions.iter() {
        if azimuth > min_az && azimuth < max_az && altitude > min_alt {
            return true;
        }
    }
    false
}

/// Function to check if an object is visible using settings from config.
///
/// * `ra`: Right ascension in hour angle formatted as "hh:mm:ss"
/// * `dec`: Declination in degrees formatted as "dd:mm:ss"
/// * `date`: date of observation
pub fn is_visible(ra: &str, dec: &str, date: DateTime<Utc>) -> bool {
    let settings = Settings::new().expect("Failed to load settings");
    is_visible_with_observatory(ra, dec, date, &settings.observatory)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::settings::Observatory;
    use chrono::NaiveDateTime;

    fn la_spezia_observatory() -> Observatory {
        Observatory {
            place: "La Spezia".to_string(),
            latitude: 44.1,
            longitude: 9.8,
            altitude: 200.0,
            observatory_name: "Test".to_string(),
            observer_name: "Test".to_string(),
            mpc_code: "123".to_string(),
            north_altitude: 10,
            south_altitude: 10,
            east_altitude: 10,
            west_altitude: 10,
        }
    }

    fn utc_datetime(iso: &str) -> DateTime<Utc> {
        let naive_dt =
            NaiveDateTime::parse_from_str(iso, "%Y-%m-%d %H:%M:%S").expect("Invalid date string");
        DateTime::from_naive_utc_and_offset(naive_dt, Utc)
    }

    #[test]
    fn test_convert_hour_angle_to_dec() {
        assert!((convert_hour_angle_to_dec("12:0:0") - 180.0).abs() < 1e-6);
        assert!((convert_hour_angle_to_dec("0:0:0") - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_convert_dec_to_deg() {
        assert!((convert_dec_to_deg("+45:30:0") - 45.5).abs() < 1e-6);
        assert!((convert_dec_to_deg("-10:0:0") - (-10.0)).abs() < 1e-6);
    }

    #[test]
    fn test_is_visible_with_observatory_fixture_object() {
        let observatory = la_spezia_observatory();
        let test_date = utc_datetime("2025-01-15 00:00:00");
        assert!(is_visible_with_observatory(
            "04:58:06",
            "+29:30:18",
            test_date,
            &observatory
        ));
    }

    #[test]
    fn test_is_not_visible_low_declination() {
        let observatory = la_spezia_observatory();
        let test_date = utc_datetime("2000-01-01 00:00:00");
        assert!(!is_visible_with_observatory(
            "12:0:0",
            "-80:0:0",
            test_date,
            &observatory
        ));
    }
}
