use crate::i18n::{self, keys};
use crate::settings::{default_mpc_auth_token, General, Observatory, Settings};
use crate::tui;
use anyhow::Context;
use promkit::{
    crossterm::{
        execute,
        style::Color,
        terminal::{disable_raw_mode, Clear, ClearType},
    },
    preset::form::Form,
    preset::listbox::Listbox,
    preset::readline::Readline,
    style::StyleBuilder,
    text_editor,
};
use std::convert::TryFrom;
use std::num::ParseIntError;

const OPTIONS_GENERAL_SETTINGS: [&str; 3] = ["1", "9", "0"];

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
fn validate_settings_menu_option(option: &str) -> bool {
    create_menu_validator(&OPTIONS_GENERAL_SETTINGS)(option)
}

// Funzione per generare il messaggio di errore
fn generate_settings_menu_error_message(option: &str) -> String {
    create_menu_error_generator(&OPTIONS_GENERAL_SETTINGS)(option)
}

/// Creates and prints general settings menu, asking for prompt
pub fn general_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n{} {}
1. Language
9. {}
0. {}",
        i18n::t(keys::GENERAL),
        i18n::t(keys::SETTINGS_MENU_TITLE),
        i18n::t(keys::BACK),
        i18n::t(keys::QUIT)
    );
    let mut p = Readline::default()
        .title(&i18n::t(keys::SELECT_OPTION))
        .validator(
            validate_settings_menu_option,
            generate_settings_menu_error_message,
        )
        .prompt()?;
    let result = p.run()?;
    match result.as_str() {
        "1" => language_menu()?,
        "9" => tui::settings_menu()?,
        _ => (),
    }
    Ok(())
}

/// Creates and prints language menu, asking for option
fn language_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\nLanguage Settings");
    let mut p = Listbox::new(vec!["en", "it"])
        .title("Select language / Seleziona lingua:")
        .listbox_lines(5)
        .prompt()?;
    let mut set = Settings::new()
        .map_err(|e| format!("Failed to load settings: {}", e))?;
    set.set_lang(p.run().context("Failed to read language selection")?)
        .map_err(|e| format!("Failed to save language: {}", e))?;
    println!("Language saved successfully / Lingua salvata con successo");
    Ok(())
}

impl TryFrom<Vec<&str>> for Settings {
    type Error = ParseIntError;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        //     if value.len() != 11 {
        //     return Err(ParseIntError("Invalid number of elements"));
        // }

        let actual_settings: Settings = Settings::new().unwrap();

        let general = General {
            lang: "".to_string(),
            mpc_auth_token: default_mpc_auth_token(),
        };

        let observatory = Observatory {
            place: if value[0].is_empty() {
                actual_settings.get_place().to_string()
            } else {
                value[0].to_string()
            },
            latitude: if value[1].is_empty() {
                *actual_settings.get_latitude()
            } else {
                value[1].parse::<f32>().unwrap()
            }, // value[1].parse::<f32>().unwrap(),
            longitude: if value[2].is_empty() {
                *actual_settings.get_longitude()
            } else {
                value[2].parse::<f32>().unwrap()
            },
            altitude: if value[3].is_empty() {
                *actual_settings.get_altitude()
            } else {
                value[3].parse::<f32>().unwrap()
            },
            observatory_name: if value[4].is_empty() {
                actual_settings.get_observatory_name().to_string()
            } else {
                value[4].to_string()
            },
            observer_name: if value[5].is_empty() {
                actual_settings.get_observer_name().to_string()
            } else {
                value[5].to_string()
            },
            mpc_code: if value[6].is_empty() {
                actual_settings.get_mpc_code().to_string()
            } else {
                value[6].to_string()
            },
            north_altitude: if value[7].is_empty() {
                *actual_settings.get_north_altitude()
            } else {
                value[7].parse::<i32>()?
            },
            south_altitude: if value[8].is_empty() {
                *actual_settings.get_south_altitude()
            } else {
                value[8].parse::<i32>()?
            },
            east_altitude: if value[9].is_empty() {
                *actual_settings.get_east_altitude()
            } else {
                value[9].parse::<i32>()?
            },
            west_altitude: if value[10].is_empty() {
                *actual_settings.get_west_altitude()
            } else {
                value[10].parse::<i32>()?
            },
        };

        Ok(Settings {
            general,
            observatory,
        })
    }
}

/// Creates a text editor state for a form field
fn create_form_field(prefix: String) -> text_editor::State {
    text_editor::State {
        texteditor: Default::default(),
        history: Default::default(),
        prefix,
        mask: Default::default(),
        prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
        active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
        inactive_char_style: StyleBuilder::new().build(),
        edit_mode: Default::default(),
        word_break_chars: Default::default(),
        lines: Default::default(),
    }
}

/// Validates latitude value (-90 to 90)
fn validate_latitude(lat: &str) -> bool {
    lat.parse::<f32>()
        .map(|l| l >= -90.0 && l <= 90.0)
        .unwrap_or(false)
}

/// Validates longitude value (-180 to 180)
fn validate_longitude(lon: &str) -> bool {
    lon.parse::<f32>()
        .map(|l| l >= -180.0 && l <= 180.0)
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_latitude() {
        assert!(validate_latitude("0"));
        assert!(validate_latitude("45.5"));
        assert!(validate_latitude("-45.5"));
        assert!(validate_latitude("90"));
        assert!(validate_latitude("-90"));
        
        assert!(!validate_latitude("91"));
        assert!(!validate_latitude("-91"));
        assert!(!validate_latitude("abc"));
        assert!(!validate_latitude(""));
    }

    #[test]
    fn test_validate_longitude() {
        assert!(validate_longitude("0"));
        assert!(validate_longitude("120.5"));
        assert!(validate_longitude("-120.5"));
        assert!(validate_longitude("180"));
        assert!(validate_longitude("-180"));
        
        assert!(!validate_longitude("181"));
        assert!(!validate_longitude("-181"));
        assert!(!validate_longitude("abc"));
        assert!(!validate_longitude(""));
    }
}

/// Creates and prints observatory settings menu, asking for prompt
pub fn observatory_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    let actual_settings = Settings::new()
        .map_err(|e| format!("Failed to load settings: {}", e))?;
    println!("\n\n\nObservatory Settings");
    let mut p = Form::new([
        create_form_field(format!("Place Name ({}): ", actual_settings.get_place())),
        create_form_field(format!("Latitude ({}): ", actual_settings.get_latitude())),
        create_form_field(format!("Longitude ({}): ", actual_settings.get_longitude())),
        create_form_field(format!("Altitude ({}): ", actual_settings.get_altitude())),
        create_form_field(format!(
            "Observatory Name: ({}): ",
            actual_settings.get_observatory_name()
        )),
        create_form_field(format!("Observer Name: ({}): ", actual_settings.get_observer_name())),
        create_form_field(format!("MPC Code: ({}): ", actual_settings.get_mpc_code())),
        create_form_field(format!(
            "North Altitude ({}): ",
            actual_settings.get_north_altitude()
        )),
        create_form_field(format!(
            "South Altitude ({}): ",
            actual_settings.get_south_altitude()
        )),
        create_form_field(format!("East Altitude ({}): ", actual_settings.get_east_altitude())),
        create_form_field(format!("West Altitude ({}): ", actual_settings.get_west_altitude())),
    ])
    .prompt()?;
    let response = p.run()?;
    
    // Validate latitude and longitude if provided
    if !response[1].is_empty() {
        if !validate_latitude(&response[1]) {
            eprintln!("Warning: Latitude must be between -90 and 90 degrees");
        }
    }
    if !response[2].is_empty() {
        if !validate_longitude(&response[2]) {
            eprintln!("Warning: Longitude must be between -180 and 180 degrees");
        }
    }
    
    let new_vec: Vec<&str> = response.iter().map(|s| s.as_str()).collect();
    let mut settings = Settings::try_from(new_vec)
        .map_err(|e| format!("Failed to parse settings: {}", e))?;
    
    settings.set_settings(settings.clone())
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    println!("Settings saved successfully / Impostazioni salvate con successo");

    Ok(())
}
