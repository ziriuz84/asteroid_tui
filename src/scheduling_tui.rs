use crate::{
    settings::General, settings::Observatory, settings::Settings, tui, weather, weather::Forecast,
};
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use promkit::{
    crossterm::{
        self, cursor, execute,
        style::Color,
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    },
    preset::form::Form,
    preset::listbox::Listbox,
    preset::readline::Readline,
    style::StyleBuilder,
    suggest::Suggest,
    text_editor,
};
use std::convert::TryFrom;
use std::num::ParseIntError;

use comfy_table::Table;

fn parse_input(input: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_dt = NaiveDateTime::parse_from_str(input, "%Y%m%d%H%M")?;
    Ok(DateTime::<Utc>::from_utc(naive_dt, Utc))
}

fn add_hours(dt: DateTime<Utc>, hours: u32) -> DateTime<Utc> {
    dt + Duration::hours(hours.into())
}

fn format_output(dt: DateTime<Utc>) -> String {
    let items = StrftimeItems::new("%d-%m %H");
    dt.format_with_items(items).to_string()
}

fn create_weather_table() {
    disable_raw_mode();
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
        "Time",
        "Cloud Cover",
        "Seeing",
        "Transparency",
        "Lifted Index",
        "RH2m",
        "Wind",
        "Temp",
        "Prec",
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

const SCHEDULING: [&str; 3] = ["1", "9", "0"];

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

pub fn scheduling_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\nScheduling Menu
1. Weather Forecast
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
    let mut result = p.run()?;
    match result.as_str() {
        "1" => create_weather_table(),
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

pub fn weather_forecast() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
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
    let mut result = p.run()?;
    match result.as_str() {
        "9" => tui::settings_menu()?,
        _ => (),
    }
    Ok(())
}
