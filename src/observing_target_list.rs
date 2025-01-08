use crate::settings::Settings;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::fmt::Display;

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
    pub magnitude: u8,
    /// Object altitude
    pub altitude: u8,
}

pub struct WhatsUpParams {
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    duration: String,
    max_objects: String,
    min_alt: String,
    solar_elong: String,
    lunar_elong: String,
    object_type: String,
}

impl Default for WhatsUpParams {
    fn default() -> Self {
        let mut params: WhatsUpParams = WhatsUpParams {
            year: "2025".to_string(),
            month: "1".to_string(),
            day: "9".to_string(),
            minute: "0".to_string(),
            hour: "0".to_string(),
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

fn get_observing_target_list(params: &WhatsUpParams) -> String {
    let mut observing_target_list: Vec<PossibleTarget> = Vec::new();
    let settings = Settings::new().unwrap();
    let mut full_params: Vec<(&str, &str)> = Vec::new();
    full_params.push(("utf8", "%E2%9C%93"));
    full_params.push((
        "authenticity_token",
        "W5eBzzw9Clj4tJVzkz0z%2F2EK18jvSS%2BffHxZpAshylg%3D",
    ));
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
    println!("{:?}", full_params);
    let url: reqwest::Url = reqwest::Url::parse_with_params(
        "https://www.minorplanetcenter.net/whatsup/index",
        full_params,
    )
    .unwrap();
    println!("{}", url);
    let result = reqwest::blocking::get(url)
        .expect("Failed on api call")
        .text()
        .expect("Failed to convert to text");
    println!("{}", result);
    result
}

pub fn parse_whats_up_response(params: &WhatsUpParams) -> Result<Vec<PossibleTarget>> {
    let data: Vec<PossibleTarget> =
        serde_json::from_str(&get_observing_target_list(params).as_str()).unwrap();
    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_observing_target_list() {
        assert!(get_observing_target_list(&WhatsUpParams::default()).contains("Designation"));
    }

    //fn test_parse_whats_up_response() {
    //    assert!(parse_whats_up_response("").is_ok());
    //}
}
