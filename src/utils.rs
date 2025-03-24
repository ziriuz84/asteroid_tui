//! # Utils
//!
//! Library for general utilities

use crate::settings::Settings;
//use astro;
use astronav::{
    coords::{dms_to_deg, hms_to_deg, star::AltAzBuilder},
    time::{gmst_in_degrees, julian_day_number, julian_time, lmst_in_degrees},
};
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc};
use std::f64::consts::PI;

fn convert_hour_angle_to_radians(ra: String) -> f64 {
    let splitted_ra: Vec<&str> = ra.split(' ').collect();
    let new_deg: f64 = splitted_ra[0].parse().unwrap();
    let new_min: f64 = splitted_ra[1].parse().unwrap();
    let new_sec: f64 = splitted_ra[2].parse().unwrap();

    ((new_deg * 15.0) + (new_min * 0.25) + (new_sec * (15.0 / 3600.0))) * (PI / 180.0)
}

fn convert_dec_to_radians(ra: String) -> f64 {
    let splitted_dec: Vec<&str> = ra.split(' ').collect();
    let new_deg: f64 = splitted_dec[0].parse().unwrap();
    let new_min: f64 = splitted_dec[1].parse().unwrap();
    let new_sec: f64 = splitted_dec[2].parse().unwrap();

    (new_deg + (new_min / 60.0) + (new_sec / 3600.0)) * (PI / 180.0)
}

fn convert_hour_angle_to_dec(ra: String) -> f64 {
    let splitted_ra: Vec<&str> = ra.split(' ').collect();
    let new_deg: f64 = splitted_ra[0].parse().unwrap();
    let new_min: f64 = splitted_ra[1].parse().unwrap();
    let new_sec: f64 = splitted_ra[2].parse().unwrap();

    (new_deg * 15.0) + (new_min * 0.25) + (new_sec * (15.0 / 3600.0))
}

fn convert_dec_to_deg(dec: String) -> f64 {
    let splitted_ra: Vec<&str> = dec.split(' ').collect();
    let new_deg: f64 = splitted_ra[0].parse().unwrap();
    let new_min: f64 = splitted_ra[1].parse().unwrap();
    let new_sec: f64 = splitted_ra[2].parse().unwrap();

    new_deg + (new_min / 60.0) + (new_sec / 3600.0)
}

fn convert_deg_to_radians(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

/// Function to check if an object is visible
///
/// * `ra`: Right ascension in hour angle formatted as "hh:mm:ss"
/// * `dec`: Declination in degrees formatted as "dd:mm:ss"
/// * `date`: date of observation
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
    println!("Altitude: {}", altitude);
    println!("Azimuth: {}", azimuth);
    if !(45.0..=315.0).contains(&azimuth) && altitude > *settings.get_north_altitude() as f64 {
        return true;
    };
    if azimuth > 45.0 && azimuth < 135.0 && altitude > *settings.get_south_altitude() as f64 {
        return true;
    }
    if azimuth > 135.0 && azimuth < 225.0 && altitude > *settings.get_south_altitude() as f64 {
        return true;
    }
    if azimuth > 225.0 && azimuth < 315.0 && altitude > *settings.get_west_altitude() as f64 {
        return true;
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
