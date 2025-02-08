use crate::settings::Settings;
use chrono::{Datelike, Timelike, Utc};
use percent_encoding::percent_decode_str;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;
use std::{fmt, thread::current};

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

fn get_observing_target_list(params: &WhatsUpParams) -> String {
    let mut observing_target_list: Vec<PossibleTarget> = Vec::new();
    let settings = Settings::new().unwrap();
    let mut full_params: Vec<(&str, &str)> = Vec::new();
    let encoded_param = "%E2%9C%93";
    //full_params.push(("utf8", "%E2%9C%93"));
    let decoded = percent_decode_str(encoded_param)
        .decode_utf8_lossy()
        .into_owned();
    println!("{}", decoded.as_str());
    full_params.push(("utf8", decoded.as_str()));
    let auth_token = "W5eBzzw9Clj4tJVzkz0z%2F2EK18jvSS%2BffHxZpAshylg%3D";
    let decoded_auth_token = percent_decode_str(auth_token)
        .decode_utf8_lossy()
        .into_owned();
    println!("{}", decoded_auth_token.as_str());
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
    println!("{}", url);
    let client = reqwest::blocking::Client::new();
    let result = client
        .post(url)
        .send()
        .expect("Failed on api call")
        .text()
        .expect("Failed to convert to text");
    println!("{}", result);
    result
}

//pub fn parse_whats_up_response(params: &WhatsUpParams) -> Result<Vec<PossibleTarget>> {
pub fn parse_whats_up_response(params: &WhatsUpParams) -> String {
    //let data: Vec<PossibleTarget> =
    //    serde_json::from_str(&get_observing_target_list(params).as_str());
    let data = get_observing_target_list(params);
    let document = scraper::Html::parse_document(data.as_str());
    println!("{:?}", data);
    data
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
        let data = parse_whats_up_response(&WhatsUpParams::default());
        println!("{:?}", data);
        assert!(parse_whats_up_response(&WhatsUpParams::default()).contains("table"));
    }
}
