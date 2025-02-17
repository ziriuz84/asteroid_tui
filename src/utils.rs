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

pub struct EquatorialCoordinates {
    pub right_ascension: f64, // in ore
    pub declination: f64,     // in gradi
}

pub struct GeographicCoordinates {
    pub latitude: f64,  // in gradi
    pub longitude: f64, // in gradi
}

pub fn calculate_lst(datetime: &DateTime<Utc>, longitude: f64) -> f64 {
    // Calcola il numero di giorni giuliani dal 2000-01-01 12:00 UT
    let j2000 = DateTime::parse_from_rfc3339("2000-01-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let days_since_j2000 = (*datetime - j2000).num_days() as f64;

    // Calcola il numero di secoli giuliani dal J2000.0
    let t = days_since_j2000 / 36525.0;

    // Calcola GMST in gradi
    // Formula semplificata per GMST alle 0h UT
    let gmst_0h =
        100.46061837 + (36000.770053608 * t) + (0.000387933 * t * t) - (t * t * t / 38710000.0);

    // Aggiungi la rotazione della Terra dall'inizio del giorno
    let hours = datetime.hour() as f64;
    let minutes = datetime.minute() as f64;
    let seconds = datetime.second() as f64;
    let day_fraction = (hours + minutes / 60.0 + seconds / 3600.0) * 1.00273790935;

    // Aggiunge 15 gradi per ora di tempo siderale
    let gmst = gmst_0h + (day_fraction * 15.0);

    // Converti GMST in LST aggiungendo la longitudine
    let mut lst = gmst + longitude.to_degrees();

    // Normalizza a 360 gradi
    lst = lst % 360.0;
    if lst < 0.0 {
        lst += 360.0;
    }

    // Converti in radianti
    lst.to_radians()
}

pub fn calculate_azimuth(
    ra: f64,  // ascensione retta in radianti
    dec: f64, // declinazione in radianti
    time: DateTime<Utc>,
) -> f64 {
    // Calcola l'angolo orario

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
    // Calcola i seni e coseni necessari
    let sin_lat = observer.latitude.sin();
    let cos_lat = observer.latitude.cos();
    let sin_dec = dec.sin();
    let cos_dec = dec.cos();
    let sin_ha = ha.sin();
    let cos_ha = ha.cos();

    // Calcola l'altezza
    let sin_alt = sin_lat * sin_dec + cos_lat * cos_dec * cos_ha;
    let alt = sin_alt.asin();

    // Calcola l'azimut
    let cos_az = (sin_dec - sin_lat * sin_alt) / (cos_lat * alt.cos());
    let mut az = cos_az.acos();

    // Correggi il quadrante dell'azimut
    if sin_ha > 0.0 {
        az = 2.0 * PI - az;
    }

    // Normalizza l'azimut tra 0 e 2π
    az %= 2.0 * PI;
    if az < 0.0 {
        az += 2.0 * PI;
    }

    az
}

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

pub fn calculate_gmst(jd: f64) -> f64 {
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

        let mut diff = lst2 - lst1;
        if diff < 0.0 {
            diff += 2.0 * PI;
        }

        // La differenza dovrebbe essere circa 361°
        assert_close(diff, 361.0_f64.to_radians());
    }

    #[test]
    fn test_normalization() {
        // Test che il risultato è sempre tra 0 e 2π
        let datetime = Utc::now();
        let longitude = 180.0_f64.to_radians(); // Caso estremo

        let lst = calculate_lst(&datetime, longitude);

        assert!(lst >= 0.0 && lst < 2.0 * PI);
    }
}
