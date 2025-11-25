//! # Observing Target List
//!
//! Library for retrieving and parsing observing target lists from the Minor Planet Center (MPC)
//!
//! This library retrieves a list of observable minor planets, comets, and other celestial objects
//! from the MPC's "What's Up" service. It parses HTML responses and filters objects based on
//! visibility criteria including altitude, solar elongation, and lunar elongation.
//!
//! It gets data from [Minor Planet Center What's Up service](https://www.minorplanetcenter.net/whatsup/index)
//! and returns a vector of `PossibleTarget` structures.
//!
//! The response is an HTML page containing a table with the following columns:
//! - Designation (object name)
//! - Magnitude
//! - Solar elongation
//! - Lunar elongation
//! - Begin/End/Maximum time, RA, Dec, and Altitude
//!
//! Example of parsed data structure:
//!
//! ```rust
//! use asteroid_tui::observing_target_list::PossibleTarget;
//!
//! let target = PossibleTarget {
//!     designation: "2024 AB".to_string(),
//!     ra: "12:34:56".to_string(),
//!     dec: "+12:34:56".to_string(),
//!     magnitude: 18.5,
//!     altitude: 45.2,
//! };
//! ```
//!
//! Data can be retrieved with:
//!
//! ```rust
//! use asteroid_tui::observing_target_list::{parse_whats_up_response, WhatsUpParams};
//!
//! let params = WhatsUpParams::default();
//! let targets = parse_whats_up_response(&params);
//! for target in targets {
//!     println!("{}: RA={}, Dec={}, Mag={}",
//!              target.designation, target.ra, target.dec, target.magnitude);
//! }
//! ```

use crate::{settings::Settings, utils::is_visible};
use anyhow::{anyhow, Result};
use chrono::{Datelike, TimeZone, Timelike, Utc};
use percent_encoding::percent_decode_str;
use reqwest;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_repr::{Deserialize_repr, Serialize_repr};
//use std::fmt::Display;
//use std::{fmt, thread::current};

/// Indices of table columns from whatsup.html:
pub mod table_indices {
    /// Object designation
    pub const DESIGNATION: usize = 0;
    /// Object magnitude
    pub const MAGNITUDE: usize = 1;
    /// Solar elongation
    pub const SOLAR_ELONG: usize = 2;
    /// Lunar elongation
    pub const LUNAR_ELONG: usize = 3;
    /// Begin time
    pub const BEGIN_TIME: usize = 4;
    /// Begin right ascension
    pub const BEG_RA: usize = 5;
    /// Begin declination
    pub const BEG_DEC: usize = 6;
    /// Begin altitude
    pub const BEG_ALT: usize = 7;
    /// Maximum time
    pub const MAX_TIME: usize = 8;
    /// Maximum right ascension
    pub const MAX_RA: usize = 9;
    /// Maximum declination
    pub const MAX_DEC: usize = 10;
    /// Maximum altitude
    pub const MAX_ALT: usize = 11;
    /// End time
    pub const END_TIME: usize = 12;
    /// End right ascension
    pub const END_RA: usize = 13;
    /// End declination
    pub const END_DEC: usize = 14;
    /// End altitude
    pub const END_ALT: usize = 15;
}

/// Structure representing a possible observing target
///
/// Contains the essential information needed to identify and observe a celestial object,
/// including its designation, coordinates, brightness, and altitude at the observation time.
///
/// # Fields
///
/// * `designation`: Object designation (e.g., "2024 AB", "C/2024 A1")
/// * `ra`: Right ascension in format "HH MM SS" or "HH:MM:SS"
/// * `dec`: Declination in format "±DD MM SS" or "±DD:MM:SS"
/// * `magnitude`: Apparent visual magnitude (lower values are brighter)
/// * `altitude`: Altitude above horizon in degrees at the observation time
///
/// # Example
///
/// ```rust
/// use asteroid_tui::observing_target_list::PossibleTarget;
///
/// let target = PossibleTarget {
///     designation: "2024 AB".to_string(),
///     ra: "12:34:56".to_string(),
///     dec: "+45:30:15".to_string(),
///     magnitude: 18.5,
///     altitude: 45.2,
/// };
///
/// println!("Target: {} at RA={}, Dec={}, Mag={:.1}, Alt={:.1}°",
///          target.designation, target.ra, target.dec, target.magnitude, target.altitude);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PossibleTarget {
    /// Object designation
    pub designation: String,
    /// Object RA
    pub ra: String,
    /// Object Dec
    pub dec: String,
    /// Object magnitude
    pub magnitude: f32,
    /// Object altitude
    pub altitude: f32,
}

/// Request parameters for querying the MPC What's Up service
///
/// All parameters are stored as strings to match the API format. Use `Default::default()`
/// to create parameters with current date/time and reasonable defaults, or construct
/// manually for custom queries.
///
/// # Fields
///
/// * `year`: Year of scheduled observation (4 digits, e.g., "2024")
/// * `month`: Month of scheduled observation (1-12, e.g., "3")
/// * `day`: Day of scheduled observation (1-31, e.g., "27")
/// * `hour`: Hour of scheduled observation (0-23, e.g., "20")
/// * `minute`: Minutes of scheduled observation (0-59, e.g., "0")
/// * `duration`: Duration of scheduled observation in hours (e.g., "1")
/// * `max_objects`: Maximum number of objects to retrieve (e.g., "10")
/// * `min_alt`: Minimum altitude of object in degrees (e.g., "10")
/// * `solar_elong`: Minimum solar elongation in degrees (e.g., "0")
/// * `lunar_elong`: Minimum lunar elongation in degrees (e.g., "0")
/// * `object_type`: Object type filter ("mp" for minor planets, "comet" for comets, etc.)
///
/// # Example
///
/// ```rust
/// use asteroid_tui::observing_target_list::WhatsUpParams;
///
/// // Use default (current time)
/// let params = WhatsUpParams::default();
///
/// // Or create custom parameters
/// let custom_params = WhatsUpParams {
///     year: "2024".to_string(),
///     month: "3".to_string(),
///     day: "27".to_string(),
///     hour: "20".to_string(),
///     minute: "0".to_string(),
///     duration: "2".to_string(),
///     max_objects: "20".to_string(),
///     min_alt: "15".to_string(),
///     solar_elong: "30".to_string(),
///     lunar_elong: "45".to_string(),
///     object_type: "mp".to_string(),
/// };
/// ```
#[derive(Debug)]
pub struct WhatsUpParams {
    /// Year of scheduled observation
    pub year: String,
    /// Month of scheduled observation
    pub month: String,
    /// Day of scheduled observation
    pub day: String,
    /// Hour of scheduled observation
    pub hour: String,
    /// Minute of scheduled observation
    pub minute: String,
    /// Duration of scheduled observation
    pub duration: String,
    /// Maximum number of object to retrieve
    pub max_objects: String,
    /// Minimum Altitude of object
    pub min_alt: String,
    /// Minimum Solar elongation
    pub solar_elong: String,
    /// Minimum Lunar elongation
    pub lunar_elong: String,
    /// Object type
    pub object_type: String,
}

impl Default for WhatsUpParams {
    fn default() -> Self {
        let current_datetime = Utc::now();
        let params: WhatsUpParams = WhatsUpParams {
            year: current_datetime.year().to_string(),
            month: current_datetime.month().to_string(),
            day: current_datetime.day().to_string(),
            minute: current_datetime.minute().to_string(),
            hour: current_datetime.hour().to_string(),
            duration: "1".to_string(),
            max_objects: "10".to_string(),
            min_alt: "10".to_string(),
            solar_elong: "0".to_string(),
            lunar_elong: "0".to_string(),
            object_type: "mp".to_string(),
        };
        params
    }
}

impl Default for PossibleTarget {
    fn default() -> Self {
        PossibleTarget {
            designation: "None".to_string(),
            ra: "None".to_string(),
            dec: "None".to_string(),
            magnitude: 0.0,
            altitude: 0.0,
        }
    }
}

/// Gets raw observing target list from MPC
///
/// * `params`: WhatsupParams struct with all requested parameters
fn get_observing_target_list(params: &WhatsUpParams) -> String {
    let settings = Settings::new().unwrap();
    let mut full_params: Vec<(&str, &str)> = Vec::new();
    let encoded_param = "%E2%9C%93";
    //full_params.push(("utf8", "%E2%9C%93"));
    let decoded = percent_decode_str(encoded_param)
        .decode_utf8_lossy()
        .into_owned();
    full_params.push(("utf8", decoded.as_str()));
    let auth_token = "W5eBzzw9Clj4tJVzkz0z%2F2EK18jvSS%2BffHxZpAshylg%3D";
    let decoded_auth_token = percent_decode_str(auth_token)
        .decode_utf8_lossy()
        .into_owned();
    full_params.push(("authenticity_token", decoded_auth_token.as_str()));
    let latitude = settings.get_latitude().to_string();
    full_params.push(("latitude", latitude.as_str()));
    let longitude = settings.get_longitude().to_string();
    full_params.push(("longitude", longitude.as_str()));
    full_params.push(("year", params.year.as_str()));
    full_params.push(("month", params.month.as_str()));
    full_params.push(("day", params.day.as_str()));
    full_params.push(("hour", params.hour.as_str()));
    full_params.push(("minute", params.minute.as_str()));
    full_params.push(("duration", params.duration.as_str()));
    full_params.push(("max_objects", params.max_objects.as_str()));
    full_params.push(("min_alt", params.min_alt.as_str()));
    full_params.push(("solar_elong", params.solar_elong.as_str()));
    full_params.push(("lunar_elong", params.lunar_elong.as_str()));
    full_params.push(("object_type", params.object_type.as_str()));
    full_params.push(("submit", "Submit"));
    let url: reqwest::Url = reqwest::Url::parse_with_params(
        "https://www.minorplanetcenter.net/whatsup/index",
        full_params,
    )
    .expect("Failed to create url");
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .send()
        .expect("Failed on api call")
        .text()
        .expect("Failed to convert to text")
}


/// Retrieves and parses the observing target list from the MPC What's Up service
///
/// This function fetches the HTML page from the MPC service, parses the table of objects,
/// and filters them based on visibility criteria (altitude constraints from settings).
/// Only objects that are actually visible from the observatory location are returned.
///
/// # Arguments
///
/// * `params`: `WhatsUpParams` struct containing all query parameters (date, time, filters, etc.)
///
/// # Returns
///
/// A vector of `PossibleTarget` structures representing objects that are:
/// - Listed in the MPC response
/// - Meet the specified criteria (altitude, elongation, etc.)
/// - Are actually visible from the observatory location at the specified time
///
/// # Errors
///
/// This function may panic if:
/// - The settings cannot be loaded
/// - The API request fails
/// - The HTML structure doesn't match the expected format
/// - Date/time parsing fails
///
/// # Example
///
/// ```rust
/// use asteroid_tui::observing_target_list::{parse_whats_up_response, WhatsUpParams};
///
/// let params = WhatsUpParams {
///     year: "2024".to_string(),
///     month: "3".to_string(),
///     day: "27".to_string(),
///     hour: "20".to_string(),
///     minute: "0".to_string(),
///     duration: "1".to_string(),
///     max_objects: "10".to_string(),
///     min_alt: "15".to_string(),
///     solar_elong: "0".to_string(),
///     lunar_elong: "0".to_string(),
///     object_type: "mp".to_string(),
/// };
///
/// let targets = parse_whats_up_response(&params);
/// println!("Found {} visible targets", targets.len());
/// for target in targets {
///     println!("  {}: Mag={:.1}, Alt={:.1}°", target.designation, target.magnitude, target.altitude);
/// }
/// ```
pub fn parse_whats_up_response(params: &WhatsUpParams) -> Vec<PossibleTarget> {
    let mut objects: Vec<PossibleTarget> = Vec::new();
    let data = get_observing_target_list(params);
    let document = scraper::Html::parse_document(data.as_str());
    let table_item_selector = scraper::Selector::parse("td").unwrap();
    let rows_selector =
        scraper::Selector::parse("#main table:nth-child(1) tr:not(:first-child)").unwrap();
    let rows: Vec<scraper::ElementRef<'_>> = document.select(&rows_selector).collect();
    rows.into_iter().for_each(|row| {
        let cells: Vec<scraper::ElementRef<'_>> = row.select(&table_item_selector).collect();
        let object: PossibleTarget =
            create_possible_target(cells).expect("Failed to create object");
        let date = Utc
            .with_ymd_and_hms(
                params.year.parse().unwrap(),
                params.month.parse().unwrap(),
                params.day.parse().unwrap(),
                params.hour.parse().unwrap(),
                params.minute.parse().unwrap(),
                0,
            )
            .unwrap();
        if is_visible(
            &object.ra.replace(" ", ":"),
            &object.dec.replace(" ", ":"),
            date,
        ) {
            objects.push(object);
        }
    });
    objects
}

fn create_possible_target(item: Vec<scraper::ElementRef<'_>>) -> Result<PossibleTarget> {
    let mut possible_target = PossibleTarget::default();

    // Verifica che ci siano abbastanza elementi
    if item.len() < 8 {
        return Err(anyhow!("Not enough elements in input vector"));
    }

    let designation_selector =
        scraper::Selector::parse("a").map_err(|e| anyhow!("Failed to parse selector: {}", e))?;

    let designation = item[table_indices::DESIGNATION]
        .select(&designation_selector)
        .next()
        .ok_or_else(|| anyhow!("Designation element not found"))?;

    possible_target.designation = designation.inner_html();

    possible_target.magnitude = item[table_indices::MAGNITUDE]
        .inner_html()
        .parse::<f32>()
        .map_err(|e| anyhow!("Failed to parse magnitude: {}", e))?;

    possible_target.altitude = item[table_indices::BEG_ALT]
        .inner_html()
        .replace(' ', "")
        .parse::<f32>()
        .map_err(|e| anyhow!("Failed to parse altitude: {}", e))?;

    possible_target.ra = item[table_indices::BEG_RA].inner_html();
    possible_target.dec = item[table_indices::BEG_DEC].inner_html();

    Ok(possible_target)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_observing_target_list() {
        let result = get_observing_target_list(&WhatsUpParams::default());
        assert!(result.contains("Designation"));
    }

    #[test]
    fn test_parse_whats_up_response() {
        assert!(!parse_whats_up_response(&WhatsUpParams::default()).is_empty());
    }
}
