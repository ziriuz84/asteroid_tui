use crate::settings::Settings;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Wind10m {
    pub direction: String,
    pub speed: Wind10mVelocity,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub timepoint: i8,
    #[serde(rename = "cloudcover")]
    pub cloud_cover: CloudCover,
    pub seeing: Seeing,
    pub transparency: Transparency,
    pub lifted_index: LiftedIndex,
    pub rh2m: RH2m,
    pub wind10m: Wind10m,
    pub temp2m: i8,
    pub prec_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastResponse {
    product: String,
    pub init: String,
    pub dataseries: Vec<Forecast>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Transparency {
    ZeroThree = 1,
    ZeroFour = 2,
    ZeroFive = 3,
    ZeroSix = 4,
    ZeroSeven = 5,
    ZeroEight = 6,
    One = 7,
    MoreOne = 8,
}

impl Transparency {
    pub const fn to_str(self) -> &'static str {
        match self {
            Transparency::ZeroThree => "<0.3",
            Transparency::ZeroFour => "0.3-0.4",
            Transparency::ZeroFive => "0.4-0.5",
            Transparency::ZeroSix => "0.5-0.6",
            Transparency::ZeroSeven => "0.6-0.7",
            Transparency::ZeroEight => "0.7-0.85",
            Transparency::One => "0.85-1",
            Transparency::MoreOne => ">1",
        }
    }
}

impl Display for Transparency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum LiftedIndex {
    BelowSeven = -10,
    SevenFive = -6,
    FiveThree = -4,
    ThreeZero = -1,
    ZeroFour = 2,
    FourEight = 6,
    EightEleven = 10,
    OverEleven = 15,
}

impl LiftedIndex {
    pub const fn to_str(self) -> &'static str {
        match self {
            LiftedIndex::BelowSeven => "Below -7",
            LiftedIndex::SevenFive => "-7 - -5",
            LiftedIndex::FiveThree => "-5 - -3",
            LiftedIndex::ThreeZero => "-3 - 0",
            LiftedIndex::ZeroFour => "0 - 4",
            LiftedIndex::FourEight => "4 - 8",
            LiftedIndex::EightEleven => "8 - 11",
            LiftedIndex::OverEleven => "Over 11",
        }
    }
}

impl Display for LiftedIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum RH2m {
    ZeroFive = -4,
    FiveTen = -3,
    TenFifteen = -2,
    FifteenTwenty = -1,
    TwentyTwentyFive = 0,
    TwentyFiveThirty = 1,
    ThirtyThirtyFive = 2,
    ThirtyFiveForty = 3,
    FortyFortyFive = 4,
    FortyFiveFifty = 5,
    FiftyFiftyFive = 6,
    FiftyFiveSixty = 7,
    SixtySixtyFive = 8,
    SixtyFiveSeventy = 9,
    SeventySeventyFive = 10,
    SeventyFiveEighty = 11,
    EightyEightyFive = 12,
    EightyFiveNinety = 13,
    NinetyNinetyFive = 14,
    NinetyFiveNinetyNine = 15,
    NinetyNineHundred = 16,
}

impl RH2m {
    pub const fn to_str(self) -> &'static str {
        match self {
            RH2m::ZeroFive => "0%-5%",
            RH2m::FiveTen => "5%-10%",
            RH2m::TenFifteen => "10%-15%",
            RH2m::FifteenTwenty => "15%-20%",
            RH2m::TwentyTwentyFive => "20%-25%",
            RH2m::TwentyFiveThirty => "25%-30%",
            RH2m::ThirtyThirtyFive => "30%-35%",
            RH2m::ThirtyFiveForty => "35%-40%",
            RH2m::FortyFortyFive => "40%-45%",
            RH2m::FortyFiveFifty => "45%-50%",
            RH2m::FiftyFiftyFive => "50%-55%",
            RH2m::FiftyFiveSixty => "55%-60%",
            RH2m::SixtySixtyFive => "60%-65%",
            RH2m::SixtyFiveSeventy => "65%-70%",
            RH2m::SeventySeventyFive => "70%-75%",
            RH2m::SeventyFiveEighty => "75%-80%",
            RH2m::EightyEightyFive => "80%-85%",
            RH2m::EightyFiveNinety => "85%-90%",
            RH2m::NinetyNinetyFive => "90%-95%",
            RH2m::NinetyFiveNinetyNine => "95%-99%",
            RH2m::NinetyNineHundred => "100%",
        }
    }
}

impl Display for RH2m {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Wind10mVelocity {
    BelowZeroThree = 1,
    Three = 2,
    Eight = 3,
    Ten = 4,
    Seventeen = 5,
    TwentyFour = 6,
    ThirtyTwo = 7,
    OverThirtyTwo = 8,
}

impl Wind10mVelocity {
    pub const fn to_str(self) -> &'static str {
        match self {
            Wind10mVelocity::BelowZeroThree => "Below 0.3 m/s",
            Wind10mVelocity::Three => "0.3-3.4 m/s",
            Wind10mVelocity::Eight => "3.4-8.0 m/s",
            Wind10mVelocity::Ten => "8.0-10.8 m/s",
            Wind10mVelocity::Seventeen => "10.8-17.2 m/s",
            Wind10mVelocity::TwentyFour => "17.2-24.5 m/s",
            Wind10mVelocity::ThirtyTwo => "24.5-32.6 m/s",
            Wind10mVelocity::OverThirtyTwo => "Over 32.6 m/s",
        }
    }
}

impl Display for Wind10mVelocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
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
    fn test_get_forecast() {
        assert!(get_forecast().contains("astro"));
    }

    #[test]
    fn test_prepare_data() {
        let data = prepare_data().unwrap();
        assert_eq!(data.product, "astro");
    }
}
