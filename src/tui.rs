use promkit::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, Clear, ClearType},
    },
    preset::readline::Readline,
};

use crate::i18n::{self, keys};
use crate::scheduling_tui;
use crate::settings_tui;

const OPTIONS_MAIN_MENU: [&str; 3] = ["1", "2", "0"];
const OPTIONS_SETTINGS_MENU: [&str; 4] = ["1", "2", "9", "0"];

/// Generic validator function for menu options
fn create_menu_validator<'a>(options: &'a [&'a str]) -> impl Fn(&str) -> bool + 'a {
    move |option: &str| options.contains(&option)
}

/// Generic error message generator for menu options
fn create_menu_error_generator<'a>(options: &'a [&'a str]) -> impl Fn(&str) -> String + 'a {
    move |option: &str| {
        format!(
            "{}: {}. {} {}.",
            i18n::t(keys::INVALID_OPTION),
            option,
            "Please choose between",
            options.join(", ")
        )
    }
}

// Funzione di validazione
fn validate_main_menu_option(option: &str) -> bool {
    create_menu_validator(&OPTIONS_MAIN_MENU)(option)
}

// Funzione per generare il messaggio di errore
fn generate_main_menu_error_message(option: &str) -> String {
    create_menu_error_generator(&OPTIONS_MAIN_MENU)(option)
}

// Funzione di validazione
fn validate_settings_menu_option(option: &str) -> bool {
    create_menu_validator(&OPTIONS_SETTINGS_MENU)(option)
}

// Funzione per generare il messaggio di errore
fn generate_settings_menu_error_message(option: &str) -> String {
    create_menu_error_generator(&OPTIONS_SETTINGS_MENU)(option)
}

/// Creates and prints main menu, asking for prompt
pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n{}
1. {}
2. {}
0. {}",
        i18n::t(keys::MAIN_MENU_TITLE),
        i18n::t(keys::SETTINGS),
        i18n::t(keys::SCHEDULING),
        i18n::t(keys::QUIT)
    );
    let mut p = Readline::default()
        .title(&i18n::t(keys::SELECT_OPTION))
        .validator(validate_main_menu_option, generate_main_menu_error_message)
        .prompt()?;
    let result = p.run()?;
    match result.as_str() {
        "1" => settings_menu()?,
        "2" => scheduling_tui::scheduling_menu()?,
        _ => (),
    }
    Ok(())
}

/// Creates and prints settings menu, asking for option
pub fn settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n{}
1. {}
2. {}
9. {}
0. {}",
        i18n::t(keys::SETTINGS_MENU_TITLE),
        i18n::t(keys::GENERAL),
        i18n::t(keys::OBSERVATORY),
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
        "1" => {
            settings_tui::general_settings_menu()?;
            settings_menu()?
        }
        "2" => {
            settings_tui::observatory_settings_menu()?;
            settings_menu()?
        }
        "9" => main_menu()?,
        _ => (),
    }
    Ok(())
}
