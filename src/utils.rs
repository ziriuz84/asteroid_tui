use crate::settings::Settings;
//use astro;
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
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

/// Calculates the local sidereal time (LST)
///
/// * `datetime`: DateTime<Utc> object
/// * `longitude`: longitude of the observer in degrees
pub fn calculate_lst(datetime: &DateTime<Utc>, longitude: f64) -> f64 {
    let j2000 = DateTime::parse_from_rfc3339("2000-01-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let days_since_j2000 = (*datetime - j2000).num_days() as f64;

    let t = days_since_j2000 / 36525.0;

    let gmst_0h =
        100.46061837 + (36000.770053608 * t) + (0.000387933 * t * t) - (t * t * t / 38710000.0);

    let hours = datetime.hour() as f64;
    let minutes = datetime.minute() as f64;
    let seconds = datetime.second() as f64;
    let day_fraction = (hours + minutes / 60.0 + seconds / 3600.0) * 1.00273790935;

    let gmst = gmst_0h + (day_fraction * 15.0);

    let mut lst = gmst + longitude.to_degrees();

    lst = lst % 360.0;
    if lst < 0.0 {
        lst += 360.0;
    }

    lst.to_radians()
}

/// Calculates the azimuth of an object
///
/// * `ra`: Right ascension in radians
/// * `dec`: Declination in radians
/// * `time`: Time in UTC
pub fn calculate_azimuth(ra: f64, dec: f64, time: DateTime<Utc>) -> f64 {
    let settings_a = Settings::new();
    let settings_b = Settings::new();
    let observer: GeographicCoordinates = GeographicCoordinates {
        latitude: *settings_a
            .expect("Error in loading settings")
            .get_latitude() as f64,
        longitude: *settings_b
            .expect("Error in loading settings")
            .get_longitude() as f64,
    };
    let lst = calculate_lst(&time, observer.longitude);
    let ha = lst - ra;
    let sin_lat = observer.latitude.sin();
    let cos_lat = observer.latitude.cos();
    let sin_dec = dec.sin();
    let cos_dec = dec.cos();
    let sin_ha = ha.sin();
    let cos_ha = ha.cos();

    let sin_alt = sin_lat * sin_dec + cos_lat * cos_dec * cos_ha;
    let alt = sin_alt.asin();

    let cos_az = (sin_dec - sin_lat * sin_alt) / (cos_lat * alt.cos());
    let mut az = cos_az.acos();

    if sin_ha > 0.0 {
        az = 2.0 * PI - az;
    }

    az %= 2.0 * PI;
    if az < 0.0 {
        az += 2.0 * PI;
    }

    az
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

    #[test]
    fn test_j2000_epoch() {
        // Test per J2000.0 (1 gennaio 2000, 12:00 UT)
        let datetime = DateTime::parse_from_rfc3339("2000-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let longitude = 0.0_f64.to_radians(); // Greenwich

        let lst = calculate_lst(&datetime, longitude);
        // Il LST a Greenwich dovrebbe essere circa 18h 41m (280.15 gradi)
        assert_close(lst, 280.46_f64.to_radians());
    }

    #[test]
    fn test_different_longitudes() {
        let datetime = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);

        // Test per longitudine est (Positiva)
        let lst_east = calculate_lst(&datetime, 45.0_f64.to_radians());

        // Test per longitudine ovest (Negativa)
        let lst_west = calculate_lst(&datetime, (-45.0_f64).to_radians());

        // La differenza dovrebbe essere 90 gradi (in radianti)
        assert_close((lst_east - lst_west).abs(), PI / 2.0);
    }

    #[test]
    fn test_known_value() {
        // Test con un valore noto:
        // 15 Giugno 2024, 22:30:00 UT, longitudine 9° Est
        let datetime = Utc.ymd(2024, 6, 15).and_hms(22, 30, 0);
        let longitude = 9.0_f64.to_radians();

        let lst = calculate_lst(&datetime, longitude);
        // Valore pre-calcolato (puoi verificare con software astronomico)
        let expected_lst = 251.1941_f64.to_radians(); // Sostituisci con il valore corretto

        assert_close(lst, expected_lst);
    }

    #[test]
    fn test_24_hour_cycle() {
        // Test che il LST aumenta di circa 361° in 24 ore
        // (più di 360° a causa della rotazione della Terra attorno al Sole)
        let datetime1 = Utc.ymd(2024, 1, 1).and_hms(0, 0, 0);
        let datetime2 = Utc.ymd(2024, 1, 2).and_hms(0, 0, 0);
        let longitude = 0.0_f64.to_radians();

        let lst1 = calculate_lst(&datetime1, longitude);
        let lst2 = calculate_lst(&datetime2, longitude);
        println!("lst1: {}, lst2: {}", lst1, lst2);

        let mut diff = lst2 - lst1;
        if diff < 0.0 {
            diff += 2.0 * PI;
        }

        // La differenza dovrebbe essere circa 361°
        assert_close(diff, 1.0_f64.to_radians());
    }

    #[test]
    fn test_normalization() {
        // Test che il risultato è sempre tra 0 e 2π
        let datetime = Utc::now();
        let longitude = 180.0_f64.to_radians(); // Caso estremo

        let lst = calculate_lst(&datetime, longitude);

        assert!(lst >= 0.0 && lst < 2.0 * PI);
    }

    #[test]
    fn test_known_star_position() {
        // Test con Vega (α Lyrae) per una data specifica
        let ra: f64 = (18.0 + 37. / 60.0 + 47.6 / 3600.0) * 15.0_f64.to_radians(); // 18h 36m 56.3s
        let dec: f64 = ((38.0 + 48.0 / 60.0 + 20.4 / 3600.0) as f64).to_radians(); // +38° 47' 01"
        let time = Utc.ymd(2024, 7, 1).and_hms(22, 0, 0); // 1 Luglio 2024, 22:00 UT

        let az = calculate_azimuth(ra, dec, time);
        println!("Azimuth for Vega: {} {}", az, az.to_degrees());
        println!("Expected: {}", 66.4794_f64.to_radians());

        // Valore pre-calcolato per Milano in quella data e ora
        let expected_az = 66.4794_f64.to_radians(); // Sostituire con il valore corretto
        assert_close(az, expected_az);
    }

    #[test]
    fn test_celestial_pole() {
        // Test con un oggetto al polo nord celeste
        let ra = 0.0; // RA non influisce per oggetti al polo
        let dec = PI / 2.0; // +90 gradi
        let time = Utc::now();

        let az = calculate_azimuth(ra, dec, time);

        // L'azimut dovrebbe essere 0 (nord) per un oggetto al polo
        assert_close(az, 0.0);
    }

    #[test]
    fn test_normalization_2() {
        // Test che l'azimut sia sempre tra 0 e 2π
        let ra = 12.0 * 15.0_f64.to_radians(); // 12h RA
        let dec = 0.0; // 0° Dec
        let time = Utc::now();

        let az = calculate_azimuth(ra, dec, time);

        assert!(az >= 0.0 && az < 2.0 * PI);
    }

    #[test]
    fn test_meridian_crossing() {
        // Test di un oggetto che attraversa il meridiano
        let time = Utc::now();
        let lst = calculate_lst(&time, 9.0_f64.to_radians()); // Milano longitude
        let ra = lst; // Oggetto sul meridiano
        let dec = 45.0_f64.to_radians(); // Declinazione uguale alla latitudine di Milano

        let az = calculate_azimuth(ra, dec, time);

        // L'oggetto dovrebbe essere esattamente a sud (180°)
        assert_close(az, PI);
    }

    #[test]
    fn test_different_declinations() {
        // Test con diverse declinazioni per la stessa RA
        let ra = 0.0;
        let time = Utc::now();

        let az1 = calculate_azimuth(ra, 30.0_f64.to_radians(), time);
        let az2 = calculate_azimuth(ra, -30.0_f64.to_radians(), time);

        // L'oggetto nell'emisfero sud dovrebbe avere un azimut maggiore
        assert!(az2 > az1);
    }

    #[test]
    fn test_east_west() {
        // Test di oggetti a est e ovest
        let time = Utc::now();
        let lst = calculate_lst(&time, 9.0_f64.to_radians());

        // Oggetto 6h prima del meridiano (est)
        let ra_east = lst + 6.0 * 15.0_f64.to_radians();
        let az_east = calculate_azimuth(ra_east, 0.0, time);

        // Oggetto 6h dopo il meridiano (ovest)
        let ra_west = lst - 6.0 * 15.0_f64.to_radians();
        let az_west = calculate_azimuth(ra_west, 0.0, time);

        // L'oggetto a est dovrebbe avere azimut < 180°, quello a ovest > 180°
        assert!(az_east < PI);
        assert!(az_west > PI);
    }
}
