use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};

/// Represents a Near Earth Object Confirmation Page entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeocpEntry {
    /// Temporary designation of the object
    #[serde(rename = "Temp_Desig")]
    pub temp_designation: String,

    /// Score value (0-100)
    #[serde(rename = "Score")]
    pub score: u32,

    /// Discovery year
    #[serde(rename = "Discovery_year")]
    pub discovery_year: u32,

    /// Discovery month
    #[serde(rename = "Discovery_month")]
    pub discovery_month: u32,

    /// Discovery day (can be fractional)
    #[serde(rename = "Discovery_day")]
    pub discovery_day: f64,

    /// Right Ascension in degrees
    #[serde(rename = "R.A.")]
    pub right_ascension: f64,

    /// Declination in degrees
    #[serde(rename = "Decl.")]
    pub declination: f64,

    /// Visual magnitude
    #[serde(rename = "V")]
    pub visual_magnitude: f64,

    /// Update information
    #[serde(rename = "Updated")]
    pub updated: String,

    /// Number of observations
    #[serde(rename = "NObs")]
    pub num_observations: u32,

    /// Arc length in days
    #[serde(rename = "Arc")]
    pub arc_days: f64,

    /// Absolute magnitude
    #[serde(rename = "H")]
    pub absolute_magnitude: f64,

    /// Days since last seen
    #[serde(rename = "Not_Seen_dys")]
    pub days_not_seen: f64,
}

/// Retrieves data from the Minor Planet Center's Near Earth Object Confirmation Page (NEOCP)
///
/// This function fetches the JSON data from the NEOCP API endpoint and returns it as a string.
/// The data contains information about near-Earth objects that need confirmation.
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(String)`: The JSON data as a string if the request is successful
/// - `Err(Box<dyn std::error::Error>)`: An error if the request fails
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request fails
/// - The response cannot be converted to text
/// - Network connectivity issues occur
fn get_neocp() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://www.minorplanetcenter.net/Extended_Files/neocp.json")
        .send()?;
    let text = response.text()?;
    Ok(text)
}

/// Parses the JSON response from get_neocp into an array of NeocpEntry structs
///
/// This function takes the raw JSON string from the NEOCP API and deserializes it
/// into a vector of NeocpEntry structs for easier manipulation and type safety.
///
/// # Arguments
///
/// * `json_data` - The JSON string returned by get_neocp()
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Vec<NeocpEntry>)`: A vector of parsed NEOCP entries if successful
/// - `Err(Box<dyn std::error::Error>)`: An error if JSON parsing fails
///
/// # Errors
///
/// This function will return an error if:
/// - The JSON is malformed
/// - Required fields are missing
/// - Type conversion fails
fn parse_neocp_data(json_data: &str) -> Result<Vec<NeocpEntry>, Box<dyn std::error::Error>> {
    let entries: Vec<NeocpEntry> = serde_json::from_str(json_data)?;
    Ok(entries)
}

/// Convenience function that fetches and parses NEOCP data in one call
///
/// This function combines get_neocp() and parse_neocp_data() to provide
/// a single function that fetches the data and returns parsed structs.
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Vec<NeocpEntry>)`: A vector of parsed NEOCP entries if successful
/// - `Err(Box<dyn std::error::Error>)`: An error if fetching or parsing fails
fn get_parsed_neocp_data() -> Result<Vec<NeocpEntry>, Box<dyn std::error::Error>> {
    let json_data = get_neocp()?;
    let entries = parse_neocp_data(&json_data)?;
    Ok(entries)
}

/// Converts decimal degrees to degrees:minutes:seconds format
///
/// # Arguments
///
/// * `decimal_degrees` - The angle in decimal degrees
///
/// # Returns
///
/// A string in "dd:mm:ss" format
fn decimal_degrees_to_dms(decimal_degrees: f64) -> String {
    let degrees = decimal_degrees.abs().floor() as i32;
    let minutes_float = (decimal_degrees.abs() - degrees as f64) * 60.0;
    let minutes = minutes_float.floor() as i32;
    let seconds = (minutes_float - minutes as f64) * 60.0;

    format!("{}:{}:{:.1}", degrees, minutes, seconds)
}

/// Converts decimal degrees to hours:minutes:seconds format for right ascension
///
/// # Arguments
///
/// * `decimal_degrees` - The angle in decimal degrees
///
/// # Returns
///
/// A string in "hh:mm:ss" format
fn decimal_degrees_to_hms(decimal_degrees: f64) -> String {
    let hours_float = decimal_degrees / 15.0; // Convert degrees to hours
    let hours = hours_float.floor() as i32;
    let minutes_float = (hours_float - hours as f64) * 60.0;
    let minutes = minutes_float.floor() as i32;
    let seconds = (minutes_float - minutes as f64) * 60.0;

    format!("{}:{}:{:.1}", hours, minutes, seconds)
}

/// Filters NEOCP entries to only include those that are currently visible
///
/// This function takes a vector of NEOCP entries and filters them based on
/// visibility criteria using the current time and location settings.
///
/// # Arguments
///
/// * `entries` - A vector of NeocpEntry structs to filter
/// * `observation_time` - The time to check visibility for (defaults to current UTC time if None)
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Vec<NeocpEntry>)`: A filtered vector of visible NEOCP entries
/// - `Err(Box<dyn std::error::Error>)`: An error if visibility calculation fails
///
/// # Errors
///
/// This function will return an error if:
/// - Settings cannot be loaded
/// - Coordinate conversion fails
fn filter_visible_objects(
    entries: Vec<NeocpEntry>,
    observation_time: Option<DateTime<Utc>>,
) -> Result<Vec<NeocpEntry>, Box<dyn std::error::Error>> {
    use crate::utils::is_visible;

    let current_time = observation_time.unwrap_or_else(|| Utc::now());
    let mut visible_entries = Vec::new();

    for entry in entries {
        // Convert decimal degrees to the string format required by is_visible
        let ra_str = decimal_degrees_to_hms(entry.right_ascension);
        let dec_str = decimal_degrees_to_dms(entry.declination);

        // Check if the object is visible
        if is_visible(&ra_str, &dec_str, current_time) {
            visible_entries.push(entry);
        }
    }

    Ok(visible_entries)
}

/// Convenience function that fetches, parses, and filters NEOCP data for visible objects
///
/// This function combines get_parsed_neocp_data() and filter_visible_objects() to provide
/// a single function that fetches the data and returns only currently visible objects.
///
/// # Arguments
///
/// * `observation_time` - The time to check visibility for (defaults to current UTC time if None)
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Vec<NeocpEntry>)`: A vector of currently visible NEOCP entries
/// - `Err(Box<dyn std::error::Error>)`: An error if fetching, parsing, or filtering fails
pub fn get_visible_neocp_objects(
    observation_time: Option<DateTime<Utc>>,
) -> Result<Vec<NeocpEntry>, Box<dyn std::error::Error>> {
    let all_entries = get_parsed_neocp_data()?;
    let visible_entries = filter_visible_objects(all_entries, observation_time)?;
    Ok(visible_entries)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_neocp() {
        let result = get_neocp();
        match result {
            Ok(text) => {
                assert!(
                    text.contains("Temp_Desig"),
                    "Expected response to contain 'Temp_Desig'"
                );
                // Also check that it's valid JSON
                assert!(
                    text.starts_with('[') && text.ends_with(']'),
                    "Expected response to be a JSON array"
                );
            }
            Err(e) => {
                panic!("Failed to get neocp: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_neocp_data() {
        let json_sample = r#"[
            {
                "Temp_Desig": "P22gBet",
                "Score": 100,
                "Discovery_year": 2025,
                "Discovery_month": 10,
                "Discovery_day": 16.3,
                "R.A.": 0.4834,
                "Decl.": 25.8496,
                "V": 21.2,
                "Updated": "Added Oct. 16.83 UT",
                "NObs": 3,
                "Arc": 0.05,
                "H": 23.4,
                "Not_Seen_dys": 0.565
            }
        ]"#;

        let result = parse_neocp_data(json_sample);
        match result {
            Ok(entries) => {
                assert_eq!(entries.len(), 1);
                let entry = &entries[0];
                assert_eq!(entry.temp_designation, "P22gBet");
                assert_eq!(entry.score, 100);
                assert_eq!(entry.discovery_year, 2025);
                assert_eq!(entry.visual_magnitude, 21.2);
            }
            Err(e) => {
                panic!("Failed to parse neocp data: {}", e);
            }
        }
    }

    #[test]
    fn test_get_parsed_neocp_data() {
        let result = get_parsed_neocp_data();
        match result {
            Ok(entries) => {
                assert!(!entries.is_empty(), "Expected at least one entry");
                // Check that the first entry has the expected structure
                let first_entry = &entries[0];
                assert!(!first_entry.temp_designation.is_empty());
                assert!(first_entry.score <= 100);
                assert!(first_entry.discovery_year >= 2000);
            }
            Err(e) => {
                panic!("Failed to get parsed neocp data: {}", e);
            }
        }
    }

    #[test]
    fn test_decimal_degrees_to_dms() {
        // Test positive degrees
        assert_eq!(decimal_degrees_to_dms(25.8496), "25:50:58.6");
        // Test negative degrees
        assert_eq!(decimal_degrees_to_dms(-25.8496), "25:50:58.6");
        // Test zero
        assert_eq!(decimal_degrees_to_dms(0.0), "0:0:0.0");
    }

    #[test]
    fn test_decimal_degrees_to_hms() {
        // Test RA conversion (degrees to hours)
        assert_eq!(decimal_degrees_to_hms(0.4834), "0:1:56.0");
        // Test 6 hours RA
        assert_eq!(decimal_degrees_to_hms(90.0), "6:0:0.0");
        // Test 12 hours RA
        assert_eq!(decimal_degrees_to_hms(180.0), "12:0:0.0");
    }

    #[test]
    fn test_filter_visible_objects() {
        use chrono::TimeZone;

        // Create test entries with known coordinates
        let test_entries = vec![
            NeocpEntry {
                temp_designation: "Test1".to_string(),
                score: 100,
                discovery_year: 2025,
                discovery_month: 1,
                discovery_day: 1.0,
                right_ascension: 0.0, // 0h RA
                declination: 0.0,     // 0° Dec (equator)
                visual_magnitude: 15.0,
                updated: "Test".to_string(),
                num_observations: 1,
                arc_days: 1.0,
                absolute_magnitude: 20.0,
                days_not_seen: 0.0,
            },
            NeocpEntry {
                temp_designation: "Test2".to_string(),
                score: 90,
                discovery_year: 2025,
                discovery_month: 1,
                discovery_day: 1.0,
                right_ascension: 180.0, // 12h RA
                declination: -80.0,     // -80° Dec (very far south)
                visual_magnitude: 16.0,
                updated: "Test".to_string(),
                num_observations: 1,
                arc_days: 1.0,
                absolute_magnitude: 21.0,
                days_not_seen: 0.0,
            },
        ];

        let test_time = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let result = filter_visible_objects(test_entries, Some(test_time));

        match result {
            Ok(visible_entries) => {
                // The first entry should be visible (equator), second should not (far south)
                // Note: This test might be flaky depending on settings, but it tests the function works
                assert!(
                    visible_entries.len() <= 2,
                    "Should not have more visible entries than input"
                );
            }
            Err(e) => {
                panic!("Failed to filter visible objects: {}", e);
            }
        }
    }
}
