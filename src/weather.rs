use crate::settings::Settings;
use reqwest;
use std::collections::HashMap;

#[derive(Debug)]
struct Forecast {
    time: String,
    cloud_cover: String,
    seeing: String,
    transparency: String,
    lifted_index: String,
    temperature: String,
    rh2m: String,
    wind_direction: String,
    wind10m: String,
    precipitation: String,
}

fn get_cloud_cover_value(index: i8) -> Option<&'static str> {
    let cloud_cover = HashMap::from([
        (1, "0%-6%"),
        (2, "6%-19%"),
        (3, "19%-31%"),
        (4, "31%-44%"),
        (5, "44%-56%"),
        (6, "56%-69%"),
        (7, "69%-81%"),
        (8, "81%-94%"),
        (9, "94%-100%"),
    ]);
    cloud_cover.get(&index).cloned()
}

fn get_seeing_value(index: i8) -> Option<&'static str> {
    let seeing = HashMap::from([
        (1, "<0.5\""),
        (2, "0.5\"-0.75\""),
        (3, "0.75\"-1\""),
        (4, "1\"-1.25\""),
        (5, "1.25\"-1.5\""),
        (6, "1.5\"-2\""),
        (7, "2\"-2.5\""),
        (8, ">2.5\""),
    ]);
    seeing.get(&index).cloned()
}

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

fn get_forecast() -> String {
    let settings = Settings::new().unwrap();
    let url: reqwest::Url = reqwest::Url::parse_with_params(
        "http://www.7timer.info/bin/api.pl",
        [
            ("latitude", settings.get_latitude().to_string()),
            ("longitude", settings.get_longitude().to_string()),
            ("product", "astro".to_string()),
            ("output", "json".to_string()),
        ],
    )
    .unwrap();
    let result = reqwest::blocking::get(url).unwrap().text();
    result.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_dict_values() {
        if let Some(test) = get_cloud_cover_value(3) {
            assert_eq!(test, "19%-31%");
        } else {
            assert!(panic!());
        }
        if let Some(test) = get_seeing_value(3) {
            assert_eq!(test, "0.75\"-1\"");
        } else {
            assert!(panic!());
        }
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
        assert_eq!(get_forecast());
    }
}
