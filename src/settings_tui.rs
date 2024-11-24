use crate::{settings::General, settings::Observatory, settings::Settings, tui};
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

// Funzione di validazione
fn validate_settings_menu_option(option: &str) -> bool {
    OPTIONS_GENERAL_SETTINGS.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_settings_menu_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        OPTIONS_GENERAL_SETTINGS.join(", ")
    )
}

/// Creates and prints general settings menu, asking for prompt
pub fn general_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\nGeneral Settings
1. Language
9. Back
0. Quit"
    );
    let mut p = Readline::default()
        .title("Select an option:")
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
    let mut p = Listbox::new(vec!["en"])
        .title("Select language:")
        .listbox_lines(5)
        .prompt()?;
    let mut set: Settings = Settings::new().unwrap();
    set.set_lang(p.run().unwrap());
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

/// Creates and prints observatory settings menu, asking for prompt
pub fn observatory_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    let actual_settings: Settings = Settings::new().unwrap();
    println!("\n\n\nObservatory Settings");
    let mut p = Form::new([
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("Place Name ({}): ", actual_settings.get_place()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("Latitude ({}): ", actual_settings.get_latitude()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("Longitude ({}): ", actual_settings.get_longitude()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("Altitude ({}): ", actual_settings.get_altitude()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!(
                "Observatory Name: ({}): ",
                actual_settings.get_observatory_name()
            ),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("Observer Name: ({}): ", actual_settings.get_observer_name()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("MPC Code: ({}): ", actual_settings.get_mpc_code()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!(
                "North Altitude ({}): ",
                actual_settings.get_north_altitude()
            ),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!(
                "South Altitude ({}): ",
                actual_settings.get_south_altitude()
            ),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("East Altitude ({}): ", actual_settings.get_east_altitude()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: format!("West Altitude ({}): ", actual_settings.get_west_altitude()),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().fgc(Color::Red).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
    ])
    .prompt()?;
    let response = p.run()?;
    let mut new_vec: Vec<&str> = Vec::new();
    for s in &response {
        new_vec.push(s);
    }
    let mut settings: Settings = Settings::try_from(new_vec)?;
    let _ = settings.set_settings(settings.clone());

    println!("Parsed settings: {:?}", settings);

    Ok(())
}
