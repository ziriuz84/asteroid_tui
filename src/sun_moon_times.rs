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
//! use asteroid_tui::sun_moon_times;
//! let data = sun_moon_times::prepare_data().unwrap();
//! ```

#![warn(missing_docs)]

use crate::settings::Settings;
use anyhow::{Context, Result};
use reqwest;
use serde::Deserialize;
use serde_json;

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
    /// Sunrise time
    pub sunrise: String,
    /// Sunset time
    pub sunset: String,
    /// Solar noon time
    pub solar_noon: String,
    /// Day length
    pub day_length: String,
    /// Civil twilight begin time
    pub civil_twilight_begin: String,
    /// Civil twilight end time
    pub civil_twilight_end: String,
    /// Nautical twilight begin time
    pub nautical_twilight_begin: String,
    /// Nautical twilight end time
    pub nautical_twilight_end: String,
    /// Astronomical twilight begin time
    pub astronomical_twilight_begin: String,
    /// Astronomical twilight end time
    pub astronomical_twilight_end: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
/// Response structure from sunrise-sunset.org
///
/// * `results`: results of type SunMoonTimes
/// * `status`: status of response
/// * `tzid`: tzid setted (UTC)
pub struct SunMoonTimesResponse {
    /// Results with data
    pub results: SunMoonTimes,
    /// API status (e.g. `"OK"`)
    pub status: String,
    /// Timezone specified
    pub tzid: String,
}

/// Returns a text string with reponse from sunrise-sunset.org
fn get_sun_moon_times() -> Result<String> {
    let settings = Settings::new()
        .context("Failed to load settings")?;
    let url = reqwest::Url::parse_with_params(
        "https://api.sunrise-sunset.org/json",
        [
            ("lat", settings.observatory.latitude.to_string()),
            ("lng", settings.observatory.longitude.to_string()),
        ],
    )
    .context("Failed to parse sunrise-sunset URL")?;
    let response = reqwest::blocking::get(url)
        .context("Failed to fetch sun/moon times")?
        .text()
        .context("Failed to read sun/moon times response")?;
    Ok(response)
}

/// Parses a sunrise-sunset.org JSON response string.
pub fn parse_sun_moon_json(response: &str) -> Result<SunMoonTimesResponse> {
    serde_json::from_str(response).context("Failed to parse sun/moon times JSON")
}

/// Returns a json with data for Sunset, sunrise, etc
pub fn prepare_data() -> Result<SunMoonTimesResponse> {
    parse_sun_moon_json(&get_sun_moon_times()?)
}

#[cfg(all(test, feature = "network-tests"))]
mod test {
    use super::*;

    #[cfg(feature = "network-tests")]
    #[test]
    fn test_get_sun_moon_times_live() {
        let result = get_sun_moon_times();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("solar_noon"));
    }

    #[cfg(feature = "network-tests")]
    #[test]
    fn test_prepare_data_live() {
        let data = prepare_data();
        assert!(data.is_ok());
        let response = data.unwrap();
        assert_eq!(response.status, "OK");
    }
}
