use crate::observing_target_list::PossibleTarget;
use crate::{
    observing_target_list::parse_whats_up_response, observing_target_list::WhatsUpParams,
    sun_moon_times, sun_moon_times::SunMoonTimesResponse, tui, weather, weather::Forecast,
};
use chrono::format::StrftimeItems;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use promkit::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, Clear, ClearType},
    },
    preset::listbox::Listbox,
    preset::readline::Readline,
};
use regex;

use comfy_table::Table;

fn parse_input(input: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_dt = NaiveDateTime::parse_from_str(input, "%Y%m%d%H%M")?;
    Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc))
}

fn add_hours(dt: DateTime<Utc>, hours: u32) -> DateTime<Utc> {
    dt + Duration::hours(hours.into())
}

fn format_output(dt: DateTime<Utc>) -> String {
    let items = StrftimeItems::new("%a %H");
    dt.format_with_items(items).to_string()
}

fn create_weather_table() {
    let _ = disable_raw_mode();
    let mut table = Table::new();
    let data = weather::prepare_data().unwrap();
    let timezero = format!("{}00", data.init);
    let forecast = data.dataseries;

    let converters: Vec<Box<dyn Fn(&Forecast) -> String>> = vec![
        Box::new(move |item: &Forecast| {
            let timezero_clone = timezero.clone();
            match parse_input(timezero_clone.as_str()) {
                Ok(result) => {
                    let new_dt = add_hours(result, item.timepoint as u32);
                    format_output(new_dt)
                }
                Err(e) => format!("Errore durante il parsing: {}", e),
            }
        }),
        Box::new(|item: &Forecast| item.cloud_cover.to_str().to_string()),
        Box::new(|item: &Forecast| item.seeing.to_str().to_string()),
        Box::new(|item: &Forecast| item.transparency.to_str().to_string()),
        Box::new(|item: &Forecast| item.lifted_index.to_str().to_string()),
        Box::new(|item: &Forecast| item.rh2m.to_str().to_string()),
        Box::new(|item: &Forecast| {
            format!(
                "{} at {}",
                item.wind10m.direction,
                item.wind10m.speed.to_str()
            )
        }),
        Box::new(|item: &Forecast| item.temp2m.to_string()),
        Box::new(|item: &Forecast| item.prec_type.clone()),
    ];

    table.set_width(80).set_header(vec![
        "Time", "Clouds", "Seeing", "Transp", "Instab", "RH2m", "Wind", "T", "Prec",
    ]);
    for item in forecast {
        let row: Vec<String> = converters
            .iter()
            .map(|converter| converter(&item))
            .collect();
        table.add_row(row);
    }
    println!("{table}");
}

fn generate_sun_moon_times_table() {
    let _ = disable_raw_mode();
    let data: SunMoonTimesResponse = sun_moon_times::prepare_data().unwrap();
    println!("All times are {}", data.tzid);
    println!("Sunrise: {}", data.results.sunrise);
    println!("Sunset: {}", data.results.sunset);
    println!("Solar noon: {}", data.results.solar_noon);
    println!("Day length: {}", data.results.day_length);
    println!(
        "Civil twilight begin: {}",
        data.results.civil_twilight_begin
    );
    println!("Civil twilight end: {}", data.results.civil_twilight_end);
    println!(
        "Nautical twilight begin: {}",
        data.results.nautical_twilight_begin
    );
    println!(
        "Nautical twilight end: {}",
        data.results.nautical_twilight_end
    );
    println!(
        "Astronomical twilight begin: {}",
        data.results.astronomical_twilight_begin
    );
    println!(
        "Astronomical twilight end: {}",
        data.results.astronomical_twilight_end
    );
}

const SCHEDULING: [&str; 5] = ["1", "2", "3", "9", "0"];

// Funzione di validazione
fn validate_scheduling_menu_option(option: &str) -> bool {
    SCHEDULING.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_scheduling_menu_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        SCHEDULING.join(", ")
    )
}

/// Scheduling Menu function
///
/// It prints Scheduling Menu and asks the user to choose an option
pub fn scheduling_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\nScheduling Menu
1. Weather Forecast
2. Sun and moon times
3. Observing target list
9. Back
0. Quit"
    );
    let mut p = Readline::default()
        .title("Select an option:")
        .validator(
            validate_scheduling_menu_option,
            generate_scheduling_menu_error_message,
        )
        .prompt()?;
    let result = p.run()?;
    match result.as_str() {
        "1" => create_weather_table(),
        "2" => generate_sun_moon_times_table(),
        "3" => observing_target_list()?,
        "9" => tui::settings_menu()?,
        _ => (),
    }
    Ok(())
}

const WEATHER_FORECAST: [&str; 2] = ["9", "0"];

// Funzione di validazione
fn validate_weather_forecast_option(option: &str) -> bool {
    WEATHER_FORECAST.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_weather_forecast_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        WEATHER_FORECAST.join(", ")
    )
}

/// Weather forecast printing
pub fn weather_forecast() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\nWeather Forecast\n\n");
    create_weather_table();
    let mut p = Readline::default()
        .title("\n9 to go back, 0 to quit:")
        .validator(
            validate_weather_forecast_option,
            generate_weather_forecast_error_message,
        )
        .prompt()?;
    let result = p.run()?;
    if result.as_str() == "9" {
        tui::settings_menu()?
    }
    Ok(())
}

/// Creates the observing target list
pub fn observing_target_list() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\nObserving Target List\n\n");
    let year: String = Readline::default()
        .title("Year (YYYY): ")
        .validator(
            |x| {
                let rex = regex::Regex::new(r"^\d{4}$").unwrap();
                rex.is_match(x)
            },
            |x| format!("{} is not a valid year", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let month: String = Readline::default()
        .title("Month (MM): ")
        .validator(
            |x| {
                let rex = regex::Regex::new(r"^\d{1,2}$").unwrap();
                rex.is_match(x)
            },
            |x| format!("{} is not a valid month", x),
        )
        .validator(
            |x| {
                let accepted_values = [
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
                ];
                accepted_values.contains(&x)
            },
            |x| format!("{} is not a valid month", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let day: String = Readline::default()
        .title("Day (DD): ")
        .validator(
            |x| {
                let rex = regex::Regex::new(r"^\d{1,2}$").unwrap();
                rex.is_match(x)
            },
            |x| format!("{} is not a valid day", x),
        )
        .validator(
            |x| {
                let accepted_values = [
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
                    "28", "29", "30", "31",
                ];
                accepted_values.contains(&x)
            },
            |x| format!("{} is not a valid day", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let hour: String = Readline::default()
        .title("Hour (HH): ")
        .validator(
            |x| {
                let rex = regex::Regex::new(r"^\d{1,2}$").unwrap();
                rex.is_match(x)
            },
            |x| format!("{} is not a valid day", x),
        )
        .validator(
            |x| {
                let accepted_values = [
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23",
                ];
                accepted_values.contains(&x)
            },
            |x| format!("{} is not a valid hour", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let minute: String = Readline::default()
        .title("Minute (MM): ")
        .validator(
            |x| {
                let rex = regex::Regex::new(r"^\d{1,2}$").unwrap();
                rex.is_match(x)
            },
            |x| format!("{} is not a valid minute", x),
        )
        .validator(
            |x| {
                let accepted_values = [
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
                    "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40",
                    "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53",
                    "54", "55", "56", "57", "58", "59",
                ];
                accepted_values.contains(&x)
            },
            |x| format!("{} is not a valid minute", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let duration: String = Readline::default()
        .title("Duration in hours (H or HH): ")
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid number", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let max_objects: String = Readline::default()
        .title("Maximum number of objects: ")
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid number", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let min_alt: String = Readline::default()
        .title("Minimum Altitude (deg): ")
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid number", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let solar_elong: String = Readline::default()
        .title("Maximum Solar elongation (deg): ")
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid number", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let lunar_elong: String = Readline::default()
        .title("Maximum Lunar elongation (deg): ")
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid number", x),
        )
        .prompt()
        .unwrap()
        .run()?;
    let object_type: String = Listbox::new(vec!["Asteroid", "NEO", "Comet"])
        .title("Select object type")
        .prompt()
        .unwrap()
        .run()?;
    let object_type_code: &str = match object_type {
        object_type if object_type.as_str() == "Asteroid" => "mp",
        object_type if object_type.as_str() == "NEO" => "neo",
        object_type if object_type.as_str() == "Comet" => "cmt",
        _ => "mp",
    };
    let whats_up_params: WhatsUpParams = WhatsUpParams {
        year,
        month,
        day,
        hour,
        minute,
        max_objects,
        duration,
        min_alt,
        solar_elong,
        lunar_elong,
        object_type: object_type_code.to_string(),
    };
    let data: Vec<PossibleTarget> = parse_whats_up_response(&whats_up_params);
    create_whats_up_list_table(data);
    let mut p = Readline::default()
        .title("\n9 to go back, 0 to quit:")
        .validator(
            validate_weather_forecast_option,
            generate_weather_forecast_error_message,
        )
        .prompt()?;
    let result = p.run()?;
    if result.as_str() == "9" {
        scheduling_menu()?
    }
    Ok(())
}

fn create_whats_up_list_table(data: Vec<PossibleTarget>) {
    let _ = disable_raw_mode();
    let mut table = Table::new();
    let converters: Vec<Box<dyn Fn(&PossibleTarget) -> String>> = vec![
        Box::new(|item: &PossibleTarget| item.designation.to_string()),
        Box::new(|item: &PossibleTarget| item.magnitude.to_string()),
        Box::new(|item: &PossibleTarget| item.ra.to_string()),
        Box::new(|item: &PossibleTarget| item.dec.to_string()),
        Box::new(|item: &PossibleTarget| item.altitude.to_string()),
    ];
    table
        .set_width(80)
        .set_header(vec!["Designation", "Magnitude", "RA", "DEC", "Altitude"]);
    for item in data {
        let row: Vec<String> = converters
            .iter()
            .map(|converter| converter(&item))
            .collect();
        table.add_row(row);
    }
    println!("{table}");
}
