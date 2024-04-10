use crate::settings::Settings;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Result};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
struct Wind10m {
    direction: String,
    speed: i32,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    timepoint: i8,
    #[serde(rename = "cloudcover")]
    cloud_cover: CloudCover,
    seeing: Seeing,
    transparency: i8,
    lifted_index: i8,
    rh2m: i8,
    wind10m: Wind10m,
    temp2m: i8,
    prec_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    product: String,
    init: String,
    dataseries: Vec<Forecast>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr)]
#[repr(u8)]
pub enum CloudCover {
    Six = 1,
    Nineteen = 2,
    ThirtyOne = 3,
    FourtyFour = 4,
    FiftyFive = 5,
    SixtyNine = 6,
    EightyOne = 7,
    NinetyFour = 8,
    OneHundred = 9,
}

impl CloudCover {
    pub const fn to_str(self) -> &'static str {
        match self {
            CloudCover::Six => "0%-6%",
            CloudCover::Nineteen => "6%-19%",
            CloudCover::ThirtyOne => "19%-31%",
            CloudCover::FourtyFour => "31%-44%",
            CloudCover::FiftyFive => "44%-56%",
            CloudCover::SixtyNine => "56%-69%",
            CloudCover::EightyOne => "69%-81%",
            CloudCover::NinetyFour => "81%-94%",
            CloudCover::OneHundred => "94%-100%",
        }
    }
}

impl Display for CloudCover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr)]
#[repr(u8)]
pub enum Seeing {
    ZeroFive = 1,
    ZeroSeven = 2,
    One = 3,
    OneTwo = 4,
    OneFive = 5,
    Two = 6,
    TwoFive = 7,
    MoreTwoFive = 8,
}

impl Seeing {
    pub const fn to_str(self) -> &'static str {
        match self {
            Seeing::ZeroFive => "<0.5\"",
            Seeing::ZeroSeven => "0.5\"-0.75\"",
            Seeing::One => "0.75\"-1\"",
            Seeing::OneTwo => "1\"-1.25\"",
            Seeing::OneFive => "1.25\"-1.5\"",
            Seeing::Two => "1.5\"-2\"",
            Seeing::TwoFive => "2\"-2.5\"",
            Seeing::MoreTwoFive => ">2.5\"",
        }
    }
}

impl Display for Seeing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

/// Returns a string with transparency values
///
/// * `index`: the index from json response
fn get_transparency_value(index: i8) -> Option<&'static str> {
    let transparency = HashMap::from([
        (1, "<0.3"),
        (2, "0.3-0.4"),
        (3, "0.4-0.5"),
        (4, "0.5-0.6"),
        (5, "0.6-0.7"),
        (6, "0.7-0.85"),
        (7, "0.85-1"),
        (8, ">1"),
    ]);
    transparency.get(&index).cloned()
}

/// Returns a string with lifted index
///
/// * `index`: the index from json response
fn get_lifted_index_value(index: i8) -> Option<&'static str> {
    let lifted_index = HashMap::from([
        (-10, "Below -7"),
        (-6, "-7 - -5"),
        (-4, "-5 - -3"),
        (-1, "-3 - 0"),
        (2, "0 - 4"),
        (6, "4 - 8"),
        (10, "8 - 11"),
        (15, "Over 11"),
    ]);
    lifted_index.get(&index).cloned()
}

/// Returns a string with rh range
///
/// * `index`: the index from json response
fn get_rh2m_value(index: i8) -> Option<&'static str> {
    let rh2m = HashMap::from([
        (-4, "0%-5%"),
        (-3, "5%-10%"),
        (-2, "10%-15%"),
        (-1, "15%-20%"),
        (0, "20%-25%"),
        (1, "25%-30%"),
        (2, "30%-35%"),
        (3, "35%-40%"),
        (4, "40%-45%"),
        (5, "45%-50%"),
        (6, "50%-55%"),
        (7, "55%-60%"),
        (8, "60%-65%"),
        (9, "65%-70%"),
        (10, "70%-75%"),
        (11, "75%-80%"),
        (12, "80%-85%"),
        (13, "85%-90%"),
        (14, "90%-95%"),
        (15, "95%-99%"),
        (16, "100%"),
    ]);
    rh2m.get(&index).cloned()
}

/// Returns a string with wind velocity
///
/// * `index`: the index from json response
fn get_wind10m_value(index: i8) -> Option<&'static str> {
    let wind10m = HashMap::from([
        (1, "Below 0.3 m/s"),
        (2, "0.3-3.4m/s"),
        (3, "3.4-8.0m/s"),
        (4, "8.0-10.8m/s"),
        (5, "10.8-17.2m/s"),
        (6, "17.2-24.5m/s"),
        (7, "24.5-32.6m/s"),
        (8, "Over 32.6m/s"),
    ]);
    wind10m.get(&index).cloned()
}

/// Returns the string with full response
fn get_forecast() -> String {
    let settings = Settings::new().unwrap();
    let url: reqwest::Url = reqwest::Url::parse_with_params(
        "http://www.7timer.info/bin/api.pl",
        [
            ("lat", settings.get_latitude().to_string()),
            ("lon", settings.get_longitude().to_string()),
            ("product", "astro".to_string()),
            ("output", "json".to_string()),
        ],
    )
    .unwrap();
    let result = reqwest::blocking::get(url).unwrap().text();
    result.unwrap()
}

/// Returns the ForecastResponse struct with data
pub fn prepare_data() -> Result<ForecastResponse> {
    let response: String = get_forecast();
    serde_json::from_str(&response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_dict_values() {
        if let Some(test) = get_transparency_value(3) {
            assert_eq!(test, "0.4-0.5");
        } else {
            assert!(panic!());
        }
        if let Some(test) = get_lifted_index_value(2) {
            assert_eq!(test, "0 - 4");
        } else {
            assert!(panic!());
        }
        if let Some(test) = get_rh2m_value(2) {
            assert_eq!(test, "30%-35%");
        } else {
            assert!(panic!());
        }
        if let Some(test) = get_wind10m_value(3) {
            assert_eq!(test, "3.4-8.0m/s");
        } else {
            assert!(panic!());
        }
    }

    #[test]
    fn test_get_forecast() {
        println!("{}", get_forecast());
        assert!(get_forecast().contains("astro"));
    }

    #[test]
    fn test_prepare_data() {
        let data = prepare_data().unwrap();
        println!("{:?}", data);
        assert_eq!(data.product, "astro");
    }
}
