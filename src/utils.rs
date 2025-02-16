use crate::settings::Settings;
use astro;
use chrono::{DateLike, DateTime, TimeLike, TimeZone, Utc};
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

struct EquatorialCoordinates {
    right_ascension: f64, // in ore
    declination: f64,     // in gradi
}

struct GeographicCoordinates {
    latitude: f64,  // in gradi
    longitude: f64, // in gradi
}

fn calculate_altitude(dec_string: String, ra_string: String, time: DateTime<Utc>) -> f64 {
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
    // Converti ascensione retta da ore a gradi
    let ra_degrees = eq_coords.right_ascension * 15.0;

    // Converti tutto in radianti
    let ra = ra_degrees * PI / 180.0;
    let dec = eq_coords.declination * PI / 180.0;
    let lat = geo_coords.latitude * PI / 180.0;

    // Calcola l'angolo orario locale
    let julian_day = time.to_julian_date();
    let gmst = calculate_gmst(julian_day);
    let lst = gmst + geo_coords.longitude / 15.0;
    let ha = lst * 15.0 * PI / 180.0 - ra;

    // Formula per calcolare l'altezza
    let sin_alt = (dec.sin() * lat.sin()) + (dec.cos() * lat.cos() * ha.cos());

    // Converti il risultato in gradi
    let altitude = sin_alt.asin() * 180.0 / PI;

    altitude
}

fn calculate_gmst(jd: f64) -> f64 {
    // Calcolo semplificato del Greenwich Mean Sidereal Time
    let t = (jd - 2451545.0) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t * t
        - t * t * t / 38710000.0;

    // Normalizza tra 0 e 360 gradi
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

        let jd = (365.25 * y).floor()
            + (30.6001 * (m + 1.0)).floor()
            + day
            + (hour + minute / 60.0 + second / 3600.0) / 24.0
            + 1720996.5
            + b;

        jd
    }
}

fn convert_dec_deg_to_radians(deg: f32) -> f64 {
    deg as f64 * (PI / 180.0)
}

#[cfg(test)]
mod test {
    use super::*;

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
        let rad = convert_deg_to_radians(deg.to_string());
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
}
