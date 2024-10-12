use crate::{settings::General, settings::Observatory, settings::Settings, tui};
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

pub fn general_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n   General Settings
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
    let mut result = p.run()?;
    match result.as_str() {
        "1" => language_menu()?,
        "9" => tui::settings_menu()?,
        _ => (),
    }
    Ok(())
}

fn language_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\n   Language Settings");
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

        let general = General {
            lang: "".to_string(),
        };

        let observatory = Observatory {
            place: value[0].to_string(),
            latitude: value[1].parse::<f32>().unwrap(),
            longitude: value[2].parse::<f32>().unwrap(),
            altitude: value[3].parse::<f32>().unwrap(),
            observatory_name: value[4].to_string(),
            observer_name: value[5].to_string(),
            mpc_code: value[6].to_string(),
            north_altitude: value[7].parse::<i32>()?,
            south_altitude: value[8].parse::<i32>()?,
            east_altitude: value[9].parse::<i32>()?,
            west_altitude: value[10].parse::<i32>()?, // Nota: stiamo usando il valore dell'est per l'ovest
        };

        Ok(Settings {
            general,
            observatory,
        })
    }
}

pub fn observatory_settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!("\n\n\n   Observatory Settings");
    let mut p = Form::new([
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: String::from("Place Name:"),
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
            prefix: String::from("Latitude:"),
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
            prefix: String::from("Longitude:"),
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
            prefix: String::from("Altitude:"),
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
            prefix: String::from("Observatory Name:"),
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
            prefix: String::from("Observer Name:"),
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
            prefix: String::from("MPC Code:"),
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
            prefix: String::from("North Altitude:"),
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
            prefix: String::from("South Altitude:"),
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
            prefix: String::from("East Altitude:"),
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
            prefix: String::from("West Altitude:"),
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
    settings.set_settings(settings.clone());

    println!("Parsed settings: {:?}", settings);

    Ok(())
}
