//! # Weather
//!
//! Library for getting astronomical weather forecasts from 7timer.info API
//!
//! This library retrieves astronomical weather data including cloud cover, seeing conditions,
//! transparency, wind, temperature, and other meteorological parameters relevant for astronomical
//! observations.
//!
//! It gets data from [7timer.info API](http://www.7timer.info/bin/api.pl) and returns a structure to be parsed.
//!
//! The API provides astronomical weather forecasts with parameters relevant for observational astronomy.
//!
//! Here is an example of the response:
//!
//! ```json
//! {
//!     "product": "astro",
//!     "init": "2024032718",
//!     "dataseries": [
//!         {
//!             "timepoint": 3,
//!             "cloudcover": 9,
//!             "seeing": 6,
//!             "transparency": 5,
//!             "lifted_index": -4,
//!             "rh2m": 11,
//!             "wind10m": {
//!                 "direction": "SE",
//!                 "speed": 2
//!             },
//!             "temp2m": 29,
//!             "prec_type": "none"
//!         }
//!     ]
//! }
//! ```
//!
//! Data can be called directly with:
//!
//! ```rust
//! use asteroid_tui::weather;
//! let data = weather::prepare_data().unwrap();
//! println!("Initial time: {}", data.init);
//! for forecast in data.dataseries {
//!     println!("Timepoint: {}, Cloud cover: {}", forecast.timepoint, forecast.cloud_cover);
//! }
//! ```

use crate::settings::Settings;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize, serde::Serialize)]
/// Wind data structure for wind at 10 meters altitude
///
/// # Fields
///
/// * `direction`: Wind direction as cardinal or intercardinal point (N, NE, E, SE, S, SW, W, NW)
/// * `speed`: Wind speed as `Wind10mVelocity` enum
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::{Wind10m, Wind10mVelocity};
///
/// let wind = Wind10m {
///     direction: "NW".to_string(),
///     speed: Wind10mVelocity::Three,
/// };
/// println!("Wind: {} at {}", wind.direction, wind.speed.to_str());
/// ```
pub struct Wind10m {
    /// Wind direction as cardinal point (N, NE, E, SE, S, SW, W, NW)
    pub direction: String,
    /// Wind speed as `Wind10mVelocity` enum
    pub speed: Wind10mVelocity,
}

#[derive(Debug, Deserialize, Serialize)]
/// Forecast data structure for a single time point
///
/// Contains all meteorological parameters relevant for astronomical observations at a specific
/// forecast time. The `timepoint` field indicates hours from the initial reference time (`init`).
///
/// # Fields
///
/// * `timepoint`: Hours from the initial reference time (from `ForecastResponse.init`)
/// * `cloud_cover`: Cloud coverage percentage range
/// * `seeing`: Astronomical seeing conditions (atmospheric turbulence)
/// * `transparency`: Atmospheric transparency (sky clarity)
/// * `lifted_index`: Atmospheric stability indicator
/// * `rh2m`: Relative humidity at 2 meters altitude
/// * `wind10m`: Wind conditions at 10 meters altitude
/// * `temp2m`: Temperature at 2 meters altitude (in Celsius)
/// * `prec_type`: Precipitation type (e.g., "none", "rain", "snow")
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::{Forecast, CloudCover, Seeing, Transparency, Wind10m, Wind10mVelocity};
///
/// let forecast = Forecast {
///     timepoint: 3,
///     cloud_cover: CloudCover::Six,
///     seeing: Seeing::One,
///     transparency: Transparency::One,
///     lifted_index: LiftedIndex::ZeroFour,
///     rh2m: RH2m::TwentyTwentyFive,
///     wind10m: Wind10m {
///         direction: "NW".to_string(),
///         speed: Wind10mVelocity::Three,
///     },
///     temp2m: 15,
///     prec_type: "none".to_string(),
/// };
/// ```
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
/// Forecast response data structure from 7timer.info API
///
/// Contains the complete forecast response with multiple time points. The `init` field
/// represents the initial reference time in format "YYYYMMDDHH" (e.g., "2024032718" for
/// March 27, 2024 at 18:00 UTC).
///
/// # Fields
///
/// * `product`: Product type (typically "astro" for astronomical forecasts)
/// * `init`: Initial reference time in format "YYYYMMDDHH" (year, month, day, hour in UTC)
/// * `dataseries`: Array of `Forecast` instances, each representing conditions at a specific
///   time point (hours from `init`)
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather;
///
/// let response = weather::prepare_data().unwrap();
/// println!("Initial time: {}", response.init); // e.g., "2024032718"
/// println!("Number of forecasts: {}", response.dataseries.len());
///
/// // Access first forecast (3 hours from init)
/// if let Some(first) = response.dataseries.first() {
///     println!("Timepoint: {} hours from init", first.timepoint);
///     println!("Cloud cover: {}", first.cloud_cover);
/// }
/// ```
pub struct ForecastResponse {
    /// Product type (typically "astro")
    product: String,
    /// Initial reference time in format "YYYYMMDDHH" (UTC)
    pub init: String,
    /// Data array with forecast values for multiple time points
    pub dataseries: Vec<Forecast>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
/// Cloud cover percentage ranges for astronomical observations
///
/// Lower values indicate clearer skies, which are better for astronomical observations.
/// Values are represented as integers (1-9) in the API response, where higher numbers
/// indicate more cloud coverage.
///
/// # Astronomical Significance
///
/// - Values 1-3 (0%-31%): Excellent to good conditions for observations
/// - Values 4-6 (31%-69%): Moderate conditions, may have partial cloud interference
/// - Values 7-9 (69%-100%): Poor conditions, significant cloud coverage
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::CloudCover;
///
/// let cover = CloudCover::Six; // 0%-6% cloud cover
/// println!("Cloud cover: {}", cover.to_str());
/// if matches!(cover, CloudCover::Six | CloudCover::Nineteen | CloudCover::ThirtyOne) {
///     println!("Good conditions for observing!");
/// }
/// ```
pub enum CloudCover {
    /// 0%-6% cloud cover - Excellent conditions
    Six = 1,
    /// 6%-19% cloud cover - Very good conditions
    Nineteen = 2,
    /// 19%-31% cloud cover - Good conditions
    ThirtyOne = 3,
    /// 31%-44% cloud cover - Moderate conditions
    FourtyFour = 4,
    /// 44%-55% cloud cover - Moderate to poor conditions
    FiftyFive = 5,
    /// 55%-69% cloud cover - Poor conditions
    SixtyNine = 6,
    /// 69%-81% cloud cover - Very poor conditions
    EightyOne = 7,
    /// 81%-94% cloud cover - Extremely poor conditions
    NinetyFour = 8,
    /// 94%-100% cloud cover - Overcast, unsuitable for observations
    OneHundred = 9,
}

impl CloudCover {
    /// Returns a human-readable string representation of the cloud cover percentage range
    ///
    /// # Returns
    ///
    /// A string slice containing the percentage range (e.g., "0%-6%", "94%-100%")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::CloudCover;
    ///
    /// let cover = CloudCover::Six;
    /// assert_eq!(cover.to_str(), "0%-6%");
    /// ```
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
/// Astronomical seeing conditions (atmospheric turbulence)
///
/// Seeing is measured in arcseconds (") and represents the angular size of a star's image
/// as affected by atmospheric turbulence. Lower values indicate better seeing conditions.
///
/// # Astronomical Significance
///
/// - Values 1-3 (<1"): Excellent to very good seeing, ideal for high-resolution observations
/// - Values 4-5 (1"-1.5"): Good seeing, suitable for most observations
/// - Values 6-7 (1.5"-2.5"): Moderate seeing, acceptable for general observations
/// - Value 8 (>2.5"): Poor seeing, significant atmospheric disturbance
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::Seeing;
///
/// let seeing = Seeing::One; // 0.75"-1" seeing
/// println!("Seeing: {}", seeing.to_str());
/// if matches!(seeing, Seeing::ZeroFive | Seeing::ZeroSeven | Seeing::One) {
///     println!("Excellent seeing conditions!");
/// }
/// ```
pub enum Seeing {
    /// <0.5" - Exceptional seeing conditions
    ZeroFive = 1,
    /// 0.5"-0.75" - Excellent seeing conditions
    ZeroSeven = 2,
    /// 0.75"-1" - Very good seeing conditions
    One = 3,
    /// 1"-1.25" - Good seeing conditions
    OneTwo = 4,
    /// 1.25"-1.5" - Moderate to good seeing conditions
    OneFive = 5,
    /// 1.5"-2" - Moderate seeing conditions
    Two = 6,
    /// 2"-2.5" - Poor seeing conditions
    TwoFive = 7,
    /// >2.5" - Very poor seeing conditions
    MoreTwoFive = 8,
}

impl Seeing {
    /// Returns a human-readable string representation of the seeing conditions
    ///
    /// # Returns
    ///
    /// A string slice containing the seeing range in arcseconds (e.g., "<0.5\"", "1\"-1.25\"")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::Seeing;
    ///
    /// let seeing = Seeing::One;
    /// assert_eq!(seeing.to_str(), "0.75\"-1\"");
    /// ```
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
/// Atmospheric transparency (sky clarity)
///
/// Transparency is a measure of how clear the atmosphere is, affecting the ability to see
/// faint objects. Higher values indicate better transparency. The scale typically ranges
/// from 0 to 1, with values >1 indicating exceptional clarity.
///
/// # Astronomical Significance
///
/// - Values 1-3 (<0.5): Poor transparency, significant atmospheric haze or pollution
/// - Values 4-5 (0.5-0.7): Moderate transparency, acceptable for bright objects
/// - Values 6-7 (0.7-1.0): Good to excellent transparency, suitable for faint objects
/// - Value 8 (>1.0): Exceptional transparency, ideal for deep-sky observations
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::Transparency;
///
/// let transparency = Transparency::One; // 0.85-1.0
/// println!("Transparency: {}", transparency.to_str());
/// if transparency as u8 >= 6 {
///     println!("Good transparency for deep-sky observing!");
/// }
/// ```
pub enum Transparency {
    /// <0.3 - Very poor transparency
    ZeroThree = 1,
    /// 0.3-0.4 - Poor transparency
    ZeroFour = 2,
    /// 0.4-0.5 - Moderate to poor transparency
    ZeroFive = 3,
    /// 0.5-0.6 - Moderate transparency
    ZeroSix = 4,
    /// 0.6-0.7 - Moderate to good transparency
    ZeroSeven = 5,
    /// 0.7-0.85 - Good transparency
    ZeroEight = 6,
    /// 0.85-1.0 - Excellent transparency
    One = 7,
    /// >1.0 - Exceptional transparency
    MoreOne = 8,
}

impl Transparency {
    /// Returns a human-readable string representation of the transparency value
    ///
    /// # Returns
    ///
    /// A string slice containing the transparency range (e.g., "<0.3", "0.85-1")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::Transparency;
    ///
    /// let transparency = Transparency::One;
    /// assert_eq!(transparency.to_str(), "0.85-1");
    /// ```
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
/// Lifted Index - atmospheric stability indicator
///
/// The Lifted Index measures atmospheric instability. Negative values indicate unstable
/// conditions (potential for clouds and precipitation), while positive values indicate
/// stable conditions (clearer skies).
///
/// # Astronomical Significance
///
/// - Values -10 to -4 (Below -3): Very unstable, likely cloudy/stormy conditions
/// - Values -1 to 2 (-3 to 4): Unstable to neutral, possible cloud formation
/// - Values 6 to 15 (4 to Over 11): Stable conditions, generally clearer skies
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::LiftedIndex;
///
/// let index = LiftedIndex::ZeroFour; // 0-4
/// println!("Lifted Index: {}", index.to_str());
/// if index as i8 >= 2 {
///     println!("Stable conditions expected");
/// }
/// ```
pub enum LiftedIndex {
    /// Below -7 - Very unstable, severe weather likely
    BelowSeven = -10,
    /// -7 to -5 - Very unstable, stormy conditions
    SevenFive = -6,
    /// -5 to -3 - Unstable, cloudy conditions likely
    FiveThree = -4,
    /// -3 to 0 - Slightly unstable, possible clouds
    ThreeZero = -1,
    /// 0 to 4 - Neutral to slightly stable
    ZeroFour = 2,
    /// 4 to 8 - Stable conditions
    FourEight = 6,
    /// 8 to 11 - Very stable conditions
    EightEleven = 10,
    /// Over 11 - Extremely stable, clear conditions
    OverEleven = 15,
}

impl LiftedIndex {
    /// Returns a human-readable string representation of the lifted index range
    ///
    /// # Returns
    ///
    /// A string slice containing the lifted index range (e.g., "Below -7", "4 - 8")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::LiftedIndex;
    ///
    /// let index = LiftedIndex::ZeroFour;
    /// assert_eq!(index.to_str(), "0 - 4");
    /// ```
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
/// Relative humidity at 2 meters altitude
///
/// High humidity can lead to condensation on optics and reduced transparency.
/// Lower humidity values are generally better for astronomical observations.
///
/// # Astronomical Significance
///
/// - Values -4 to 0 (0%-25%): Very low humidity, excellent conditions
/// - Values 1 to 6 (25%-55%): Low to moderate humidity, good conditions
/// - Values 7 to 11 (55%-80%): Moderate to high humidity, possible dew formation
/// - Values 12 to 16 (80%-100%): Very high humidity, dew likely, poor conditions
///
/// # Example
///
/// ```rust
/// use asteroid_tui::weather::RH2m;
///
/// let rh = RH2m::TwentyTwentyFive; // 20%-25%
/// println!("Relative Humidity: {}", rh.to_str());
/// if rh as i8 <= 6 {
///     println!("Low humidity - good for observing!");
/// }
/// ```
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
    /// Returns a human-readable string representation of the relative humidity range
    ///
    /// # Returns
    ///
    /// A string slice containing the humidity percentage range (e.g., "0%-5%", "95%-99%")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::RH2m;
    ///
    /// let rh = RH2m::TwentyTwentyFive;
    /// assert_eq!(rh.to_str(), "20%-25%");
    /// ```
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
    /// Returns a human-readable string representation of the wind velocity range
    ///
    /// # Returns
    ///
    /// A string slice containing the wind speed range in m/s (e.g., "Below 0.3 m/s", "0.3-3.4 m/s")
    ///
    /// # Example
    ///
    /// ```rust
    /// use asteroid_tui::weather::Wind10mVelocity;
    ///
    /// let wind = Wind10mVelocity::Three;
    /// assert_eq!(wind.to_str(), "0.3-3.4 m/s");
    /// ```
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

/// Retrieves and parses astronomical weather forecast data from the 7timer.info API
///
/// This function fetches weather forecast data based on the observatory's latitude and longitude
/// from the application settings. The forecast includes multiple time points with detailed
/// meteorological parameters relevant for astronomical observations.
///
/// # Returns
///
/// Returns `Ok(ForecastResponse)` containing the parsed forecast data, or `Err` if the API
/// request fails or the response cannot be parsed.
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
/// use asteroid_tui::weather;
///
/// match weather::prepare_data() {
///     Ok(forecast) => {
///         println!("Initial time: {}", forecast.init);
///         for data in forecast.dataseries {
///             println!("Timepoint {}: Cloud={}, Seeing={}, Wind={}",
///                      data.timepoint,
///                      data.cloud_cover.to_str(),
///                      data.seeing.to_str(),
///                      data.wind10m.speed.to_str());
///         }
///     }
///     Err(e) => eprintln!("Failed to get forecast: {}", e),
/// }
/// ```
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
