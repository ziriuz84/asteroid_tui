use crate::settings::Settings;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize, serde::Serialize)]
/// Wind data structure for Wind at 10 m of altitude
///
/// * `direction`: direction
/// * `speed`: speed
pub struct Wind10m {
    /// Direction as cardinal point, i.e. NW, E...
    pub direction: String,
    /// Speed as Wind10mVelocity Enum
    pub speed: Wind10mVelocity,
}

#[derive(Debug, Deserialize, Serialize)]
/// Forecast data structure
///
/// * `timepoint`: time of the forecast
/// * `cloud_cover`: Cloud coverage
/// * `seeing`: Seeing
/// * `transparency`: Transparency
/// * `lifted_index`: Lifted Index
/// * `rh2m`: Rh at 2 m of altitude
/// * `wind10m`: Wind at 10 m of altitude
/// * `temp2m`: Temperature at 2 m of altitude
/// * `prec_type`: Precipitation type
pub struct Forecast {
    /// Time of the forecast (in hours from init)
    pub timepoint: i8,
    #[serde(rename = "cloudcover")]
    /// Cloud coverage as CloudCover enum
    pub cloud_cover: CloudCover,
    /// Seeing as Seeing Enum
    pub seeing: Seeing,
    /// Transparency as Transparency Enum
    pub transparency: Transparency,
    /// Lifted Index as LiftedIndex enum
    pub lifted_index: LiftedIndex,
    /// RH at 2 m as RH2m enum
    pub rh2m: RH2m,
    /// Wind at 10 m as Wind10m data structure
    pub wind10m: Wind10m,
    /// Temperature at 2 m
    pub temp2m: i8,
    /// Precipitation type
    pub prec_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
/// Forecast responsa data structure
///
/// * `product`: product type
/// * `init`: Initial reference time
/// * `dataseries`: an array of Forecast instances
pub struct ForecastResponse {
    /// Product type
    product: String,
    /// Initial reference time
    pub init: String,
    /// Data array with forecast values
    pub dataseries: Vec<Forecast>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
/// CloudCover enum
pub enum CloudCover {
    /// 0%-6%
    Six = 1,
    /// 6%-19%
    Nineteen = 2,
    /// 19%-31%
    ThirtyOne = 3,
    /// 31%-44%
    FourtyFour = 4,
    /// 44%-55%
    FiftyFive = 5,
    /// 55%-69%
    SixtyNine = 6,
    /// 69%-81%
    EightyOne = 7,
    /// 81%-94%
    NinetyFour = 8,
    /// 94%-100%
    OneHundred = 9,
}

impl CloudCover {
    /// Returns a string representation of CloudCover
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
/// Seeing enum
pub enum Seeing {
    /// <0,5
    ZeroFive = 1,
    /// 0.5-0.75
    ZeroSeven = 2,
    /// 0.75-1
    One = 3,
    /// 1-1.25
    OneTwo = 4,
    /// 1.25-1.5
    OneFive = 5,
    /// 1.5-2
    Two = 6,
    /// 2-2.5
    TwoFive = 7,
    /// >2.5
    MoreTwoFive = 8,
}

impl Seeing {
    /// Returns a string representation of Seeing
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
/// Transparency enum
pub enum Transparency {
    /// <0.3
    ZeroThree = 1,
    /// 0.3-0.4
    ZeroFour = 2,
    /// 0.4-0.5
    ZeroFive = 3,
    /// 0.5-0.6
    ZeroSix = 4,
    /// 0.6-0.7
    ZeroSeven = 5,
    /// 0.7-0.85
    ZeroEight = 6,
    /// 0.85-1
    One = 7,
    /// >1
    MoreOne = 8,
}

impl Transparency {
    /// Returns a string representation of Transparency
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
/// Lifted Index enum
pub enum LiftedIndex {
    /// Below -7
    BelowSeven = -10,
    /// -7 - -5
    SevenFive = -6,
    /// -5 - -3
    FiveThree = -4,
    /// -3 - 0
    ThreeZero = -1,
    /// 0 - 4
    ZeroFour = 2,
    /// 4 - 8
    FourEight = 6,
    /// 8 - 11
    EightEleven = 10,
    /// Over 11
    OverEleven = 15,
}

impl LiftedIndex {
    /// Returns a string representation of LiftedIndex
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
/// RH2m enum
pub enum RH2m {
    /// 0%-5%
    ZeroFive = -4,
    /// 5%-10%
    FiveTen = -3,
    /// 10%-15%
    TenFifteen = -2,
    /// 15%-20%
    FifteenTwenty = -1,
    /// 20%-25%
    TwentyTwentyFive = 0,
    /// 25%-30%
    TwentyFiveThirty = 1,
    /// 30%-35%
    ThirtyThirtyFive = 2,
    /// 35%-40%
    ThirtyFiveForty = 3,
    /// 40%-45%
    FortyFortyFive = 4,
    /// 45%-50%
    FortyFiveFifty = 5,
    /// 50%-55%
    FiftyFiftyFive = 6,
    /// 55%-60%
    FiftyFiveSixty = 7,
    /// 60%-65%
    SixtySixtyFive = 8,
    /// 65%-70%
    SixtyFiveSeventy = 9,
    /// 70%-75%
    SeventySeventyFive = 10,
    /// 75%-80%
    SeventyFiveEighty = 11,
    /// 80%-85%
    EightyEightyFive = 12,
    /// 85%-90%
    EightyFiveNinety = 13,
    /// 90%-95%
    NinetyNinetyFive = 14,
    /// 95%-99%
    NinetyFiveNinetyNine = 15,
    /// 100%
    NinetyNineHundred = 16,
}

impl RH2m {
    /// Returns a string representation of RH2m
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
/// Wind10mVelocity enum
pub enum Wind10mVelocity {
    /// Below 0.3 m/s
    BelowZeroThree = 1,
    /// 0.3-3.4 m/s
    Three = 2,
    /// 3.4-8.0 m/s
    Eight = 3,
    /// 8.0-10.8 m/s
    Ten = 4,
    /// 10.8-17.2 m/s
    Seventeen = 5,
    /// 17.2-24.5 m/s
    TwentyFour = 6,
    /// 24.5-32.6 m/s
    ThirtyTwo = 7,
    /// Over 32.6 m/s
    OverThirtyTwo = 8,
}

impl Wind10mVelocity {
    /// Returns a string representation of Wind10mVelocity
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
