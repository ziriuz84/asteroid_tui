use crate::{settings::Settings, utils::is_visible};
use anyhow::{anyhow, Context, Result};
use chrono::{Datelike, TimeZone, Timelike, Utc};
use percent_encoding::percent_decode_str;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const MPC_WHATSUP_INDEX_URL: &str = "https://www.minorplanetcenter.net/whatsup/index";
const MPC_WHATSUP_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Last-resort token if scraping fails or MPC markup changes (POST may still fail).
const MPC_WHATSUP_AUTH_TOKEN_FALLBACK: &str = "W5eBzzw9Clj4tJVzkz0z%2F2EK18jvSS%2BffHxZpAshylg%3D";

fn mpc_http_client() -> Result<reqwest::blocking::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
        ),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static(
            "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8",
        ),
    );
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(MPC_WHATSUP_REQUEST_TIMEOUT)
        .build()
        .context("Failed to build HTTP client for MPC")
}

/// Extract Rails `authenticity_token` from What's Up index HTML.
fn extract_authenticity_token_from_html(html: &str) -> String {
    let document = scraper::Html::parse_document(html);
    if let Ok(selector) = scraper::Selector::parse(r#"input[name="authenticity_token"]"#) {
        for element in document.select(&selector) {
            if let Some(value) = element.value().attr("value") {
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
    }

    if let Ok(re) = Regex::new(r#"name=["']authenticity_token["'][^>]*value=["']([^"']+)["']"#) {
        if let Some(caps) = re.captures(html) {
            if let Some(m) = caps.get(1) {
                return m.as_str().to_string();
            }
        }
    }

    if let Ok(re) = Regex::new(r#"<meta\s+name=["']csrf-token["']\s+content=["']([^"']+)["']"#) {
        if let Some(caps) = re.captures(html) {
            if let Some(m) = caps.get(1) {
                return m.as_str().to_string();
            }
        }
    }

    String::new()
}

/// GET the What's Up form page and scrape a fresh `authenticity_token`.
fn scrape_whatsup_authenticity_token() -> String {
    let client = match mpc_http_client() {
        Ok(client) => client,
        Err(_) => return String::new(),
    };
    let response = match client.get(MPC_WHATSUP_INDEX_URL).send() {
        Ok(response) if response.status().is_success() => response,
        _ => return String::new(),
    };
    let html = match response.text() {
        Ok(html) => html,
        Err(_) => return String::new(),
    };
    extract_authenticity_token_from_html(&html)
}

/// Scrape token from MPC, or use hardcoded fallback when scraping fails.
fn resolve_whatsup_authenticity_token() -> (String, bool) {
    let scraped = scrape_whatsup_authenticity_token();
    if !scraped.is_empty() {
        return (scraped, false);
    }
    (MPC_WHATSUP_AUTH_TOKEN_FALLBACK.to_string(), true)
}

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
fn get_observing_target_list(params: &WhatsUpParams) -> Result<String> {
    let settings = Settings::new().context("Failed to load settings")?;
    let mut full_params: Vec<(&str, &str)> = Vec::new();
    let encoded_param = "%E2%9C%93";
    let decoded = percent_decode_str(encoded_param)
        .decode_utf8_lossy()
        .into_owned();
    full_params.push(("utf8", decoded.as_str()));

    let (auth_token, used_fallback) = resolve_whatsup_authenticity_token();
    if used_fallback {
        eprintln!(
            "Warning: could not scrape MPC authenticity_token; using built-in fallback / \
             Avviso: impossibile recuperare authenticity_token da MPC; uso fallback incorporato"
        );
    }
    let decoded_auth_token = percent_decode_str(&auth_token)
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

    let url = reqwest::Url::parse_with_params(
        "https://www.minorplanetcenter.net/whatsup/index",
        full_params,
    )
    .context("Failed to create MPC URL")?;

    let client = mpc_http_client()?;
    let response = client
        .post(url)
        .send()
        .context("Failed to send request to MPC")?
        .text()
        .context("Failed to read MPC response")?;

    Ok(response)
}

//TODO: Add altitude filtering on different directions
//TODO: Write better documentation

/// Fetches and parses the MPC What's Up target list for the given parameters.
///
/// Scrapes a fresh `authenticity_token` from the MPC What's Up form (with a built-in
/// fallback if scraping fails). Observatory coordinates come from settings.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or the HTML response cannot be parsed.
///
/// # Examples
///
/// ```no_run
/// use asteroid_tui::observing_target_list::{WhatsUpParams, parse_whats_up_response};
///
/// let params = WhatsUpParams {
///     year: "2026".to_string(),
///     month: "5".to_string(),
///     day: "28".to_string(),
///     hour: "22".to_string(),
///     minute: "0".to_string(),
///     max_objects: "10".to_string(),
///     duration: "4".to_string(),
///     min_alt: "20".to_string(),
///     solar_elong: "90".to_string(),
///     lunar_elong: "60".to_string(),
///     object_type: "mp".to_string(),
/// };
/// let targets = parse_whats_up_response(&params)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn parse_whats_up_response(params: &WhatsUpParams) -> Result<Vec<PossibleTarget>> {
    let mut objects: Vec<PossibleTarget> = Vec::new();
    let data = get_observing_target_list(params)?;
    let document = scraper::Html::parse_document(data.as_str());

    let table_item_selector = scraper::Selector::parse("td")
        .map_err(|e| anyhow!("Failed to parse table item selector: {:?}", e))?;
    let rows_selector = scraper::Selector::parse("#main table:nth-child(1) tr:not(:first-child)")
        .map_err(|e| anyhow!("Failed to parse rows selector: {:?}", e))?;

    let rows: Vec<scraper::ElementRef<'_>> = document.select(&rows_selector).collect();

    // Parse date once for all objects
    let date = Utc
        .with_ymd_and_hms(
            params.year.parse().context("Failed to parse year")?,
            params.month.parse().context("Failed to parse month")?,
            params.day.parse().context("Failed to parse day")?,
            params.hour.parse().context("Failed to parse hour")?,
            params.minute.parse().context("Failed to parse minute")?,
            0,
        )
        .single()
        .context("Invalid date/time")?;

    for row in rows {
        let cells: Vec<scraper::ElementRef<'_>> = row.select(&table_item_selector).collect();
        match create_possible_target(cells) {
            Ok(object) => {
                if is_visible(
                    &object.ra.replace(" ", ":"),
                    &object.dec.replace(" ", ":"),
                    date,
                ) {
                    objects.push(object);
                }
            }
            Err(e) => {
                // Log error but continue processing other objects
                eprintln!("Warning: Failed to create object: {}", e);
            }
        }
    }

    Ok(objects)
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
    fn test_extract_authenticity_token_from_example_html() {
        let html = include_str!("../response_examples/whatsup.html");
        let token = extract_authenticity_token_from_html(html);
        assert_eq!(token, "6jL1Ruhw/ENf7P8I7VSi5YgwcNKf8+8ps2vvYtjf/Us=");
    }

    #[test]
    fn test_get_observing_target_list() {
        let result = get_observing_target_list(&WhatsUpParams::default());
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Designation"));
    }

    #[test]
    fn test_parse_whats_up_response() {
        let result = parse_whats_up_response(&WhatsUpParams::default());
        // Test that the function doesn't panic and returns a valid result
        // Note: The result may be empty depending on observatory settings and current data
        assert!(result.is_ok());
        let _objects = result.unwrap();
        // Just verify we got a Vec, even if empty
    }
}
