//! # Sun Moon Times
//!
//! Library for getting sunrise, sunset, and twilight times from the sunrise-sunset.org API
//!
//! This library retrieves solar and astronomical timing data including sunrise, sunset, solar noon,
//! day length, and various twilight periods (civil, nautical, and astronomical). The data is
//! calculated based on the observatory's latitude and longitude from the settings.
//!
//! It gets data from [sunrise-sunset.org API](https://sunrise-sunset.org/api) and returns a
//! structure to be parsed.
//!
//! Here is an example of the response:
//!
//! ```json
//! {
//!     "results": {
//!         "sunrise": "6:34:37 AM",
//!         "sunset": "5:35:35 PM",
//!         "solar_noon": "12:00:00 PM",
//!         "day_length": "11:01:58",
//!         "civil_twilight_begin": "6:07:00 AM",
//!         "civil_twilight_end": "6:02:35 PM",
//!         "nautical_twilight_begin": "5:35:00 AM",
//!         "nautical_twilight_end": "6:42:35 PM",
//!         "astronomical_twilight_begin": "5:02:00 AM",
//!         "astronomical_twilight_end": "7:15:35 PM"
//!     },
//!     "status": "OK",
//!     "tzid": "UTC"
//! }
//! ```
//!
//! Data can be retrieved with:
//!
//! ```rust
//! use asteroid_tui::sun_moon_times;
//!
//! let data = sun_moon_times::prepare_data().unwrap();
//! println!("Sunrise: {}", data.results.sunrise);
//! println!("Sunset: {}", data.results.sunset);
//! println!("Day length: {}", data.results.day_length);
//! println!("Astronomical twilight begins: {}", data.results.astronomical_twilight_begin);
//! ```

#![warn(missing_docs)]

use crate::settings::Settings;
use reqwest;
use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize, serde::Serialize)]
/// Structure containing solar and twilight timing data
///
/// All times are returned as strings in 12-hour format (e.g., "6:34:37 AM").
///
/// # Fields
///
/// * `sunrise`: Local sunrise time
/// * `sunset`: Local sunset time
/// * `solar_noon`: Time when the sun reaches its highest point in the sky
/// * `day_length`: Duration from sunrise to sunset (format: "HH:MM:SS")
/// * `civil_twilight_begin`: Beginning of civil twilight (sun 6° below horizon)
/// * `civil_twilight_end`: End of civil twilight (sun 6° below horizon)
/// * `nautical_twilight_begin`: Beginning of nautical twilight (sun 12° below horizon)
/// * `nautical_twilight_end`: End of nautical twilight (sun 12° below horizon)
/// * `astronomical_twilight_begin`: Beginning of astronomical twilight (sun 18° below horizon)
/// * `astronomical_twilight_end`: End of astronomical twilight (sun 18° below horizon)
///
/// # Example
///
/// ```rust
/// use asteroid_tui::sun_moon_times::SunMoonTimes;
///
/// let times = SunMoonTimes {
///     sunrise: "6:34:37 AM".to_string(),
///     sunset: "5:35:35 PM".to_string(),
///     solar_noon: "12:05:06 PM".to_string(),
///     day_length: "11:01:58".to_string(),
///     civil_twilight_begin: "6:07:00 AM".to_string(),
///     civil_twilight_end: "6:02:35 PM".to_string(),
///     nautical_twilight_begin: "5:35:00 AM".to_string(),
///     nautical_twilight_end: "6:42:35 PM".to_string(),
///     astronomical_twilight_begin: "5:02:00 AM".to_string(),
///     astronomical_twilight_end: "7:15:35 PM".to_string(),
/// };
/// ```
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
/// Response structure from sunrise-sunset.org API
///
/// # Fields
///
/// * `results`: Solar and twilight timing data
/// * `status`: Response status (typically "OK" for successful requests)
/// * `tzid`: Timezone identifier (typically "UTC")
///
/// # Example
///
/// ```rust
/// use asteroid_tui::sun_moon_times;
///
/// let response = sun_moon_times::prepare_data().unwrap();
/// assert_eq!(response.status, "OK");
/// assert_eq!(response.tzid, "UTC");
/// println!("Sunrise: {}", response.results.sunrise);
/// ```
pub struct SunMoonTimesResponse {
    /// Results with solar and twilight timing data
    pub results: SunMoonTimes,
    /// Response status from the API
    status: String,
    /// Timezone identifier (typically "UTC")
    pub tzid: String,
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
    let response = reqwest::blocking::get(url).unwrap().text();
    response.unwrap()
}

/// Retrieves and parses sunrise, sunset, and twilight data from the sunrise-sunset.org API
///
/// This function fetches solar timing data based on the observatory's latitude and longitude
/// from the application settings. The data includes sunrise, sunset, solar noon, day length,
/// and various twilight periods.
///
/// # Returns
///
/// Returns `Ok(SunMoonTimesResponse)` containing the parsed solar timing data, or
/// `Err` if the API request fails or the response cannot be parsed.
///
/// # Errors
///
/// This function will return an error if:
/// - The settings cannot be loaded
/// - The API request fails (network error, invalid URL, etc.)
/// - The response cannot be parsed as valid JSON
/// - The response structure doesn't match the expected format
///
/// # Example
///
/// ```rust
/// use asteroid_tui::sun_moon_times;
///
/// match sun_moon_times::prepare_data() {
///     Ok(data) => {
///         println!("Sunrise: {}", data.results.sunrise);
///         println!("Sunset: {}", data.results.sunset);
///         println!("Day length: {}", data.results.day_length);
///     }
///     Err(e) => eprintln!("Failed to get solar times: {}", e),
/// }
/// ```
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
