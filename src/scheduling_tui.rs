use crate::neocp::NeocpEntry;
use crate::i18n::{self, keys};
use crate::observing_target_list::PossibleTarget;
use crate::{
    neocp, observing_target_list::parse_whats_up_response, observing_target_list::WhatsUpParams,
    sun_moon_times, sun_moon_times::SunMoonTimesResponse, tui, weather, weather::Forecast,
};
use anyhow::{Context, Result};
use chrono::format::StrftimeItems;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

use promkit::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, Clear, ClearType},
    },
    preset::listbox::Listbox,
    preset::readline::Readline,
};

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

fn create_weather_table() -> Result<()> {
    let _ = disable_raw_mode();
    println!("\nFetching weather forecast... / Recupero previsioni meteo...");
    let mut table = Table::new();
    let data = weather::prepare_data()
        .map_err(|e| {
            eprintln!("Error fetching weather data / Errore nel recupero dati meteo: {}", e);
            e
        })?;
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
    Ok(())
}

fn generate_sun_moon_times_table() -> Result<()> {
    let _ = disable_raw_mode();
    println!("\nFetching sun/moon times... / Recupero orari sole/luna...");
    let data: SunMoonTimesResponse = sun_moon_times::prepare_data()
        .map_err(|e| {
            eprintln!("Error fetching sun/moon times / Errore nel recupero orari: {}", e);
            e
        })?;
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
    Ok(())
}

const SCHEDULING: [&str; 6] = ["1", "2", "3", "4", "9", "0"];

/// Generic validator function for menu options
fn create_menu_validator<'a>(options: &'a [&'a str]) -> impl Fn(&str) -> bool + 'a {
    move |option: &str| options.contains(&option)
}

/// Generic error message generator for menu options
fn create_menu_error_generator<'a>(options: &'a [&'a str]) -> impl Fn(&str) -> String + 'a {
    move |option: &str| {
        format!(
            "Invalid option: {}. Please choose between {}.",
            option,
            options.join(", ")
        )
    }
}

// Funzione di validazione
fn validate_scheduling_menu_option(option: &str) -> bool {
    create_menu_validator(&SCHEDULING)(option)
}

// Funzione per generare il messaggio di errore
fn generate_scheduling_menu_error_message(option: &str) -> String {
    create_menu_error_generator(&SCHEDULING)(option)
}

/// Scheduling Menu function
///
/// It prints Scheduling Menu and asks the user to choose an option
pub fn scheduling_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n{}
1. {}
2. {}
3. {}
9. {}
0. {}",
        i18n::t(keys::SCHEDULING_MENU_TITLE),
        i18n::t(keys::WEATHER_FORECAST),
        i18n::t(keys::SUN_MOON_TIMES),
        i18n::t(keys::OBSERVING_TARGET_LIST),
        i18n::t(keys::BACK),
        i18n::t(keys::QUIT)
    );
    let mut p = Readline::default()
        .title(&i18n::t(keys::SELECT_OPTION))
        .validator(
            validate_scheduling_menu_option,
            generate_scheduling_menu_error_message,
        )
        .prompt()?;
    let result = p.run()?;
    match result.as_str() {
        "1" => {
            if let Err(e) = create_weather_table() {
                eprintln!("Error: {}", e);
            }
        }
        "2" => {
            if let Err(e) = generate_sun_moon_times_table() {
                eprintln!("Error: {}", e);
            }
        }
        "3" => observing_target_list()?,
        "4" => create_neocp_list_table(),
        "9" => tui::settings_menu()?,
        _ => (),
    }
    Ok(())
}

const WEATHER_FORECAST: [&str; 2] = ["9", "0"];

// Funzione di validazione
fn validate_weather_forecast_option(option: &str) -> bool {
    create_menu_validator(&WEATHER_FORECAST)(option)
}

// Funzione per generare il messaggio di errore
fn generate_weather_forecast_error_message(option: &str) -> String {
    create_menu_error_generator(&WEATHER_FORECAST)(option)
}

/// Validates if a date is valid (checks if the date actually exists)
fn validate_date(year: u32, month: u32, day: u32) -> bool {
    NaiveDate::from_ymd_opt(year as i32, month, day).is_some()
}

/// Validates year input
fn validate_year(year: &str) -> bool {
    year.parse::<u32>()
        .map(|y| y >= 1900 && y <= 2200)
        .unwrap_or(false)
}

/// Validates month input (1-12)
fn validate_month(month: &str) -> bool {
    month.parse::<u32>()
        .map(|m| m >= 1 && m <= 12)
        .unwrap_or(false)
}

/// Validates day input (1-31, will be checked against month/year later)
fn validate_day(day: &str) -> bool {
    day.parse::<u32>()
        .map(|d| d >= 1 && d <= 31)
        .unwrap_or(false)
}

/// Validates hour input (0-23)
fn validate_hour(hour: &str) -> bool {
    hour.parse::<u32>()
        .map(|h| h <= 23)
        .unwrap_or(false)
}

/// Validates minute input (0-59)
fn validate_minute(minute: &str) -> bool {
    minute.parse::<u32>()
        .map(|m| m <= 59)
        .unwrap_or(false)
}

/// Reads and validates year input
fn read_year() -> Result<String> {
    let year = Readline::default()
        .title("Year (YYYY): ")
        .validator(validate_year, |x| format!("{} is not a valid year (1900-2200)", x))
        .prompt()
        .context("Failed to create year prompt")?
        .run()
        .context("Failed to read year input")?;
    Ok(year)
}

/// Reads and validates month input
fn read_month() -> Result<String> {
    let month = Readline::default()
        .title("Month (MM): ")
        .validator(validate_month, |x| format!("{} is not a valid month (1-12)", x))
        .prompt()
        .context("Failed to create month prompt")?
        .run()
        .context("Failed to read month input")?;
    Ok(month)
}

/// Reads and validates day input, checking against year and month
fn read_day(year: &str, month: &str) -> Result<String> {
    let year_num = year.parse::<u32>()
        .context("Invalid year for date validation")?;
    let month_num = month.parse::<u32>()
        .context("Invalid month for date validation")?;
    
    loop {
        let day = Readline::default()
            .title("Day (DD): ")
            .validator(validate_day, |x| format!("{} is not a valid day (1-31)", x))
            .prompt()
            .context("Failed to create day prompt")?
            .run()
            .context("Failed to read day input")?;
        
        // Validate the day against the actual month/year
        if let Ok(day_num) = day.parse::<u32>() {
            if validate_date(year_num, month_num, day_num) {
                return Ok(day);
            } else {
                eprintln!("{} is not a valid day for {}/{}", day, month, year);
                continue;
            }
        }
        return Ok(day);
    }
}

/// Reads and validates hour input
fn read_hour() -> Result<String> {
    let hour = Readline::default()
        .title("Hour (HH, 0-23): ")
        .validator(validate_hour, |x| format!("{} is not a valid hour (0-23)", x))
        .prompt()
        .context("Failed to create hour prompt")?
        .run()
        .context("Failed to read hour input")?;
    Ok(hour)
}

/// Reads and validates minute input
fn read_minute() -> Result<String> {
    let minute = Readline::default()
        .title("Minute (MM, 0-59): ")
        .validator(validate_minute, |x| format!("{} is not a valid minute (0-59)", x))
        .prompt()
        .context("Failed to create minute prompt")?
        .run()
        .context("Failed to read minute input")?;
    Ok(minute)
}

/// Reads a positive integer input
fn read_positive_integer(title: &str, field_name: &str) -> Result<String> {
    let value = Readline::default()
        .title(title)
        .validator(
            |x| x.parse::<u32>().is_ok(),
            |x| format!("{} is not a valid positive number", x),
        )
        .prompt()
        .with_context(|| format!("Failed to create {} prompt", field_name))?
        .run()
        .with_context(|| format!("Failed to read {} input", field_name))?;
    Ok(value)
}

/// Weather forecast printing
pub fn weather_forecast() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\nWeather Forecast\n\n");
    create_weather_table()?;
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

/// Maps object type string to code
fn map_object_type_to_code(object_type: &str) -> &str {
    match object_type {
        "Asteroid" => "mp",
        "NEO" => "neo",
        "Comet" => "cmt",
        _ => "mp",
    }
}

/// Reads object type from user
fn read_object_type() -> Result<String> {
    let object_type = Listbox::new(vec!["Asteroid", "NEO", "Comet"])
        .title("Select object type")
        .prompt()
        .context("Failed to create object type prompt")?
        .run()
        .context("Failed to read object type input")?;
    Ok(object_type)
}

/// Creates the observing target list
pub fn observing_target_list() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\nObserving Target List\n\n");
    
    // Read date and time inputs
    let year = read_year()?;
    let month = read_month()?;
    let day = read_day(&year, &month)?;
    let hour = read_hour()?;
    let minute = read_minute()?;
    
    // Read other parameters
    let duration = read_positive_integer("Duration in hours (H or HH): ", "duration")?;
    let max_objects = read_positive_integer("Maximum number of objects: ", "max_objects")?;
    let min_alt = read_positive_integer("Minimum Altitude (deg): ", "min_alt")?;
    let solar_elong = read_positive_integer("Maximum Solar elongation (deg): ", "solar_elong")?;
    let lunar_elong = read_positive_integer("Maximum Lunar elongation (deg): ", "lunar_elong")?;
    
    // Read object type
    let object_type = read_object_type()?;
    let object_type_code = map_object_type_to_code(&object_type);
    
    // Build parameters
    let whats_up_params = WhatsUpParams {
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
    
    // Fetch and display data
    println!("\nFetching observing target list... / Recupero lista obiettivi osservazione...");
    let data = parse_whats_up_response(&whats_up_params)
        .map_err(|e| {
            eprintln!("Error fetching observing target list / Errore nel recupero lista: {}", e);
            e
        })?;
    
    if data.is_empty() {
        println!("\nNo visible objects found for the selected criteria.");
        println!("Nessun oggetto visibile trovato per i criteri selezionati.");
    } else {
        println!("\nFound {} object(s) / Trovato/i {} oggetto/i:", data.len(), data.len());
        create_whats_up_list_table(data);
    }
    
    // Wait for user input to continue
    let mut p = Readline::default()
        .title(&format!("\n9 {} / {}, 0 {} / {}:",
            i18n::t(keys::BACK), "Indietro",
            i18n::t(keys::QUIT), "Esci"))
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

fn create_neocp_list_table() {
    let _ = disable_raw_mode();
    let data = neocp::get_visible_neocp_objects(None).unwrap_or_else(|_| Vec::new());
    let mut table = Table::new();
    let converters: Vec<Box<dyn Fn(&NeocpEntry) -> String>> = vec![
        Box::new(|item: &NeocpEntry| item.temp_designation.to_string()),
        Box::new(|item: &NeocpEntry| item.score.to_string()),
        Box::new(|item: &NeocpEntry| item.right_ascension.to_string()),
        Box::new(|item: &NeocpEntry| item.declination.to_string()),
        Box::new(|item: &NeocpEntry| item.visual_magnitude.to_string()),
        Box::new(|item: &NeocpEntry| item.days_not_seen.to_string()),
    ];

    table.set_width(80).set_header(vec![
        "Temp Designation",
        "Score",
        "RA",
        "DEC",
        "Mag",
        "Days not seen",
    ]);
    for item in data {
        let row: Vec<String> = converters
            .iter()
            .map(|converter| converter(&item))
            .collect();
        table.add_row(row);
    }
    println!("{table}");
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_date() {
        // Valid dates
        assert!(validate_date(2000, 1, 1));
        assert!(validate_date(2000, 2, 29)); // Leap year
        assert!(validate_date(2024, 12, 31));
        
        // Invalid dates
        assert!(!validate_date(2000, 2, 30)); // February doesn't have 30 days
        assert!(!validate_date(2001, 2, 29)); // Not a leap year
        assert!(!validate_date(2000, 13, 1)); // Invalid month
        assert!(!validate_date(2000, 0, 1)); // Invalid month
        assert!(!validate_date(2000, 1, 0)); // Invalid day
        assert!(!validate_date(2000, 1, 32)); // Invalid day
    }

    #[test]
    fn test_validate_year() {
        assert!(validate_year("2000"));
        assert!(validate_year("2024"));
        assert!(validate_year("1900"));
        assert!(validate_year("2200"));
        
        assert!(!validate_year("1899")); // Too old
        assert!(!validate_year("2201")); // Too new
        assert!(!validate_year("abc")); // Not a number
        assert!(!validate_year("")); // Empty
    }

    #[test]
    fn test_validate_month() {
        for i in 1..=12 {
            assert!(validate_month(&i.to_string()));
        }
        
        assert!(!validate_month("0"));
        assert!(!validate_month("13"));
        assert!(!validate_month("abc"));
        assert!(!validate_month(""));
    }

    #[test]
    fn test_validate_day() {
        for i in 1..=31 {
            assert!(validate_day(&i.to_string()));
        }
        
        assert!(!validate_day("0"));
        assert!(!validate_day("32"));
        assert!(!validate_day("abc"));
        assert!(!validate_day(""));
    }

    #[test]
    fn test_validate_hour() {
        for i in 0..=23 {
            assert!(validate_hour(&i.to_string()));
        }
        
        assert!(!validate_hour("24"));
        assert!(!validate_hour("abc"));
        assert!(!validate_hour(""));
    }

    #[test]
    fn test_validate_minute() {
        for i in 0..=59 {
            assert!(validate_minute(&i.to_string()));
        }
        
        assert!(!validate_minute("60"));
        assert!(!validate_minute("abc"));
        assert!(!validate_minute(""));
    }

    #[test]
    fn test_map_object_type_to_code() {
        assert_eq!(map_object_type_to_code("Asteroid"), "mp");
        assert_eq!(map_object_type_to_code("NEO"), "neo");
        assert_eq!(map_object_type_to_code("Comet"), "cmt");
        assert_eq!(map_object_type_to_code("Unknown"), "mp"); // Default
    }
}
