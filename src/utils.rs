use crate::settings::Settings;
use astro::angle;
use julian::Calendar;
//use astro;
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

/// Equatorial Coordinates struct
/// Represents equatoria coordinates for an object
///
/// * `right_ascension`: right ascension in hours
/// * `declination`: declination in degrees
pub struct EquatorialCoordinates {
    /// Right Ascension in hours
    pub right_ascension: f64,
    /// Declination in degrees
    pub declination: f64,
}

/// Geographic Coordinates struct
/// Represents geographic coordinates of the observer
///
/// * `latitude`: latitude in degrees
/// * `longitude`: longitude in degrees
pub struct GeographicCoordinates {
    /// Latitude in degrees
    pub latitude: f64,
    /// Longitude in degrees
    pub longitude: f64,
}

fn reduce_angle(angle: f64) -> f64 {
    let d = angle % 360.0;
    if d < 0.0 {
        return d + 360.0;
    }
    d
}

/// Calculates the local sidereal time (LST)
///
/// * `datetime`: DateTime<Utc> object
/// * `longitude`: longitude of the observer in degrees
pub fn calculate_lst(datetime: &DateTime<Utc>, longitude: f64) -> f64 {
    let cal = Calendar::GREGORIAN;
    let cal_now = cal
        .at_ymd(
            datetime.year(),
            julian::Month::try_from(datetime.month()).unwrap(),
            datetime.day(),
        )
        .unwrap();
    let jd_now = cal_now.julian_day_number();
    let t = (jd_now as f64 - 2451545.0) / 36525.0;
    println!("jd_now: {}", jd_now);
    println!("t: {}", t);
    let theta0 =
        280.46061837 + 360.98564736629 * (jd_now as f64 - 2451545.0) + (0.000387933 * t * t)
            - (t * t * t / 38710000.0);

    println!("theta0: {}", theta0);
    (reduce_angle(theta0) + longitude).to_radians()
}

/// Calculates the azimuth of an object
///
/// * `ra`: Right ascension in radians
/// * `dec`: Declination in radians
/// * `time`: Time in UTC
pub fn calculate_azimuth(ra: f64, dec: f64, time: DateTime<Utc>) -> f64 {
    // Carica un'unica configurazione
    let settings = Settings::new().expect("Error in loading settings");

    // Converti latitudine e longitudine da gradi a radianti
    let latitude_deg = *settings.get_latitude() as f64;
    let longitude_deg = *settings.get_longitude() as f64;

    let observer = GeographicCoordinates {
        latitude: latitude_deg.to_radians(),
        longitude: longitude_deg.to_radians(),
    };

    // Calcola LST usando la longitudine in gradi
    let lst = calculate_lst(&time, longitude_deg);
    let ha = lst - ra.to_radians();

    let sin_lat = observer.latitude.sin();
    let cos_lat = observer.latitude.cos();
    let sin_ha = ha.sin();
    println!("ha: {}", ha);
    println!("sin_ha: {}", sin_ha);
    let cos_ha = ha.cos();
    println!("cos_ha: {}", cos_ha);
    let tan_dec = dec.to_radians().tan();

    // Calcola numeratore e denominatore per l'azimut
    let numerator = sin_ha;
    println!("numerator: {}", numerator);
    let denominator = cos_ha * sin_lat - tan_dec * cos_lat;
    println!("denominator: {}", denominator);

    // Usa atan2 per il quadrante corretto e aggiusta l'azimut
    let mut az_rad = numerator.atan2(denominator); // Converti da sud a nord
    az_rad = az_rad.rem_euclid(2.0 * std::f64::consts::PI); // Normalizza tra 0 e 2π
    println!("Azimuth: {} {}", az_rad, az_rad.to_degrees());
    println!("Latitude: {}", settings.get_latitude());
    println!("Longitude: {}", settings.get_longitude());

    az_rad
}

/// Calculates the altitude of an object
///
/// * `dec_string`: Declination as a string (DD MM SS)
/// * `ra_string`: Right ascension as a string (HH MM SS)
/// * `time`: Time in UTC
pub fn calculate_altitude(dec_string: String, ra_string: String, time: DateTime<Utc>) -> f64 {
    let settings_a = Settings::new();
    let settings_b = Settings::new();
    let geo_coords: GeographicCoordinates = GeographicCoordinates {
        latitude: *settings_a
            .expect("Error in loading settings")
            .get_latitude() as f64,
        longitude: *settings_b
            .expect("Error in loading settings")
            .get_longitude() as f64,
    };
    let eq_coords: EquatorialCoordinates = EquatorialCoordinates {
        right_ascension: convert_hour_angle_to_dec(ra_string),
        declination: convert_dec_to_deg(dec_string),
    };
    let ra_degrees = eq_coords.right_ascension * 15.0;

    let ra = ra_degrees * PI / 180.0;
    let dec = eq_coords.declination * PI / 180.0;
    let lat = geo_coords.latitude * PI / 180.0;

    let julian_day = time.to_julian_date();
    let gmst = calculate_gmst(julian_day);
    let lst = gmst + geo_coords.longitude / 15.0;
    let ha = lst * 15.0 * PI / 180.0 - ra;

    let sin_alt = (dec.sin() * lat.sin()) + (dec.cos() * lat.cos() * ha.cos());

    sin_alt.asin() * 180.0 / PI
}

/// Calculates the Greenwich Mean Sidereal Time
///
/// * `jd`: Julian day
pub fn calculate_gmst(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t * t
        - t * t * t / 38710000.0;

    gmst % 360.0 / 15.0
}

trait JulianDate {
    fn to_julian_date(&self) -> f64;
}

impl JulianDate for DateTime<Utc> {
    fn to_julian_date(&self) -> f64 {
        let year = self.year() as f64;
        let month = self.month() as f64;
        let day = self.day() as f64;
        let hour = self.hour() as f64;
        let minute = self.minute() as f64;
        let second = self.second() as f64;

        let y = if month <= 2.0 { year - 1.0 } else { year };
        let m = if month <= 2.0 { month + 12.0 } else { month };

        let b = (y / 400.0).floor() - (y / 100.0).floor();

        (365.25 * y).floor()
            + (30.6001 * (m + 1.0)).floor()
            + day
            + (hour + minute / 60.0 + second / 3600.0) / 24.0
            + 1720996.5
            + b
    }
}

fn convert_dec_deg_to_radians(deg: f32) -> f64 {
    deg as f64 * (PI / 180.0)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};
    use julian::{Calendar, Month};
    use std::f64::consts::PI;

    #[test]
    fn test_convert_hour_angle_to_radians() {
        let hour = "12 30 30";
        let rad = convert_hour_angle_to_radians(hour.to_string());
        println!("rad: {}", rad);
        assert!(rad > 3.274);
        assert!(rad < 3.276);
    }

    #[test]
    fn test_convert_deg_to_radians() {
        let deg = "12 30 30";
        let rad = convert_dec_to_radians(deg.to_string());
        assert!(rad > 0.2183);
        assert!(rad < 0.2184);
    }

    #[test]
    fn test_convert_dec_deg_to_radians() {
        let deg = 12.0;
        let rad = convert_dec_deg_to_radians(deg);
        assert!(rad > 0.2094);
        assert!(rad < 0.2095);
    }

    const EPSILON: f64 = 0.01; // tolleranza per confronti in radianti (~0.057 gradi)

    fn assert_close(a: f64, b: f64) {
        let diff = (a - b).abs();
        if diff > EPSILON {
            panic!(
                "Values differ by {}, which is more than epsilon {}",
                diff, EPSILON
            );
        }
    }

    // calculate_lst tests

    #[test]
    fn test_lst_at_j2000_noon() {
        // Test a J2000.0 noon with longitude 0 (Greenwich)
        let datetime = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let longitude = 0.0;
        let lst = calculate_lst(&datetime, longitude);
        let expected_degrees: f64 = 280.46061837;
        assert!((lst - expected_degrees.to_radians()).abs() < 1e-6);
    }

    #[test]
    fn test_lst_with_positive_longitude() {
        // Longitude 45° Est
        let datetime = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let longitude = 45.0;
        let lst = calculate_lst(&datetime, longitude);
        let expected_degrees: f64 = (280.46061837 + 45.0) % 360.0;
        assert!((lst - expected_degrees.to_radians()).abs() < 1e-6);
    }

    #[test]
    fn test_lst_with_negative_longitude() {
        // Longitude 75° Ovest
        let datetime = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let longitude = -75.0;
        let lst = calculate_lst(&datetime, longitude);
        let expected_degrees: f64 = (280.46061837 - 75.0 + 360.0) % 360.0;
        assert!((lst - expected_degrees.to_radians()).abs() < 1e-6);
    }

    #[test]
    fn test_lst_at_another_date() {
        // Data arbitraria con longitudine 0
        let datetime = Utc.with_ymd_and_hms(2023, 10, 5, 12, 0, 0).unwrap();
        let longitude = 0.0;

        // Calcolo manuale basato sul codice corrente
        let cal = Calendar::GREGORIAN;
        let cal_now = cal.at_ymd(2023, Month::October, 5).unwrap();
        let jd_now = cal_now.julian_day_number();
        let t = (jd_now as f64 - 2451545.0) / 36525.0;
        let theta0 =
            280.46061837 + 360.98564736629 * (jd_now as f64 - 2451545.0) + 0.000387933 * t.powi(2)
                - t.powi(3) / 38710000.0;
        let expected_radians: f64 = (theta0 % 360.0) * PI / 180.0;

        let lst = calculate_lst(&datetime, longitude);
        assert!((lst - expected_radians).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_azimuth() {
        let ra = 12.0;
        let dec = 30.0;
        let time = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let azimuth = calculate_azimuth(ra, dec, time);
        assert_close(azimuth, 5.2404);
    }
}
