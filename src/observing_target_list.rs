use crate::settings::Settings;
use anyhow::{anyhow, Result};
use chrono::{Datelike, Timelike, Utc};
use percent_encoding::percent_decode_str;
use reqwest;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
//use serde_repr::{Deserialize_repr, Serialize_repr};
//use std::fmt::Display;
//use std::{fmt, thread::current};

/// Possible target structure
///
/// * `designation`: Object designation
/// * `ra`: Object RA
/// * `dec`: Object Dec
/// * `magnitude`: Object magnitude
/// * `altitude`: Object altitude
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

/// Request parameters struct
///
/// * `year`: Year of scheduled observation
/// * `month`: Month of scheduled observation
/// * `day`: Day of scheduled observation
/// * `hour`: Hour of scheduled observation
/// * `minute`: Minutes of scheduled observation
/// * `duration`: Duration of scheduled observation
/// * `max_objects`: Maximum number of object to retrieve
/// * `min_alt`: Minimum Altitude of object
/// * `solar_elong`: Minimum Solar elongation
/// * `lunar_elong`: Minimum Lunar elongation
/// * `object_type`: Object type
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

//TODO: Add altitude filtering on different directions
//TODO: Write better documentation

/// Returns data from what's up list of MPC
///
/// * `params`: WhatsupParams struct with all requested parameters
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
        objects.push(create_possible_target(cells).unwrap())
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

    let designation = item[0]
        .select(&designation_selector)
        .next()
        .ok_or_else(|| anyhow!("Designation element not found"))?;

    possible_target.designation = designation.inner_html();

    possible_target.magnitude = item[2]
        .inner_html()
        .parse::<f32>()
        .map_err(|e| anyhow!("Failed to parse magnitude: {}", e))?;

    possible_target.altitude = item[7]
        .inner_html()
        .parse::<f32>()
        .map_err(|e| anyhow!("Failed to parse altitude: {}", e))?;

    possible_target.ra = item[5].inner_html();
    possible_target.dec = item[6].inner_html();

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
