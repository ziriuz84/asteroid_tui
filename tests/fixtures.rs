//! Offline integration tests using checked-in API response fixtures.

use asteroid_tui::observing_target_list::{parse_whats_up_html, WhatsUpParams};
use asteroid_tui::settings::Observatory;
use asteroid_tui::sun_moon_times::parse_sun_moon_json;
use asteroid_tui::weather::parse_forecast_json;

fn fixture_observatory() -> Observatory {
    Observatory {
        place: "La Spezia".to_string(),
        latitude: 44.09727,
        longitude: 9.7737,
        altitude: 200.0,
        observatory_name: "Test".to_string(),
        observer_name: "Test".to_string(),
        mpc_code: "123".to_string(),
        north_altitude: 10,
        south_altitude: 10,
        east_altitude: 10,
        west_altitude: 10,
    }
}

fn fixture_whats_up_params() -> WhatsUpParams {
    WhatsUpParams {
        year: "2025".to_string(),
        month: "1".to_string(),
        day: "15".to_string(),
        hour: "0".to_string(),
        minute: "0".to_string(),
        duration: "1".to_string(),
        max_objects: "10".to_string(),
        min_alt: "10".to_string(),
        solar_elong: "0".to_string(),
        lunar_elong: "0".to_string(),
        object_type: "mp".to_string(),
    }
}

#[test]
fn integration_parse_7timer_fixture() {
    let json = include_str!("../response_examples/7timer.json");
    let forecast = parse_forecast_json(json).unwrap();
    assert_eq!(forecast.product, "astro");
    assert!(!forecast.dataseries.is_empty());
}

#[test]
fn integration_parse_sunrise_sunset_fixture() {
    let json = include_str!("../response_examples/sunrise_sunset.json");
    let response = parse_sun_moon_json(json).unwrap();
    assert_eq!(response.status, "OK");
    assert!(!response.results.sunrise.is_empty());
}

#[test]
fn integration_parse_whatsup_html_fixture() {
    let html = include_str!("../response_examples/whatsup.html");
    let params = fixture_whats_up_params();
    let observatory = fixture_observatory();
    let targets = parse_whats_up_html(html, &params, &observatory).unwrap();
    assert!(!targets.is_empty());
    assert!(
        targets
            .iter()
            .any(|t| t.designation.contains("Eunomia"))
    );
}
