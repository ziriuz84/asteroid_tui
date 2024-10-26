//! # Sun Moon Times
//!
//! Library for getting Sunrise, Moonrise, etc
//!
//! It gets data from api.sunrise-sunset.org and returns a structure to be parsed in some way
//!
//! Here is an example of the response:
//!
//! ```json
//! {
//!     "results": {
//!         "sunrise": "6:34:37 AM",
//!         "sunset": "5:35:35 PM",
//!         "solar_noon": "12:00:00 AM",
//!         "day_length": "05:02:00",
//!         "civil_twilight_begin": "6:07:00 AM",
//!         "civil_twilight_end": "5:32:00 PM",
//!         "nautical_twilight_begin": "6:12:00 AM",
//!         "nautical_twilight_end": "5:27:00 PM",
//!         "astronomical_twilight_begin": "6:17:00 AM",
//!         "astronomical_twilight_end": "5:22:00 PM"
//!     }
//!     "status": "OK",
//!     "tzid": "UTC"
//! }
//! ```
//!
//! data can be called directly with
//!
//! ```rust
//! use crate::sun_moon_times;
//! let data = sun_moon_times::prepare_data().unwrap();
//! ```

#![warn(missing_docs)]

use crate::settings::Settings;
use reqwest;
use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize, serde::Serialize)]
/// Structure with data for Sun, Moon, etc
///
/// * `sunrise`: Sunrise time
/// * `sunset`: Sunset time
/// * `solar_noon`: Solar noon time
/// * `day_length`: Day length
/// * `civil_twilight_begin`: Civil twilight begin time
/// * `civil_twilight_end`: Civil twilight end time
/// * `nautical_twilight_begin`: Nautical twilight begin time
/// * `nautical_twilight_end`: Nautical twilight end time
/// * `astronomical_twilight_begin`: Astronomical twilight begin time
/// * `astronomical_twilight_end`: Astronomical twilight end time
pub struct SunMoonTimes {
    sunrise: String,
    sunset: String,
    solar_noon: String,
    day_length: String,
    civil_twilight_begin: String,
    civil_twilight_end: String,
    nautical_twilight_begin: String,
    nautical_twilight_end: String,
    astronomical_twilight_begin: String,
    astronomical_twilight_end: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
/// Response structure from sunrise-sunset.org
///
/// * `results`: results of type SunMoonTimes
/// * `status`: status of response
/// * `tzid`: tzid setted (UTC)
pub struct SunMoonTimesResponse {
    results: SunMoonTimes,
    status: String,
    tzid: String,
}

/// Returns a text string with reponse from sunrise-sunset.org
fn get_sun_moon_times() -> String {
    let settings = Settings::new().unwrap();
    let url: reqwest::Url = reqwest::Url::parse_with_params(
        "https://api.sunrise-sunset.org/json",
        [
            ("lat", settings.observatory.latitude.to_string()),
            ("lng", settings.observatory.longitude.to_string()),
        ],
    )
    .unwrap();
    println!("{:?}", url);
    let response = reqwest::blocking::get(url).unwrap().text();
    println!("{:?}", response);
    response.unwrap()
}

/// Returns a json with data for Sunset, sunrise, etc
pub fn prepare_data() -> Result<SunMoonTimesResponse> {
    let response: String = get_sun_moon_times();
    serde_json::from_str(&response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_sun_moon_times() {
        assert!(get_sun_moon_times().contains("solar_noon"));
    }

    #[test]
    fn test_prepare_data() {
        let data = prepare_data().unwrap();
        assert_eq!(data.status, "OK");
    }
}
