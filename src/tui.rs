use promkit::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, Clear, ClearType},
    },
    preset::readline::Readline,
};

use crate::scheduling_tui;
use crate::settings_tui;

const OPTIONS_MAIN_MENU: [&str; 3] = ["1", "2", "0"];
const OPTIONS_SETTINGS_MENU: [&str; 4] = ["1", "2", "9", "0"];

// Funzione di validazione
fn validate_main_menu_option(option: &str) -> bool {
    OPTIONS_MAIN_MENU.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_main_menu_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        OPTIONS_MAIN_MENU.join(", ")
    )
}
// Funzione di validazione
fn validate_settings_menu_option(option: &str) -> bool {
    OPTIONS_SETTINGS_MENU.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_settings_menu_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        OPTIONS_SETTINGS_MENU.join(", ")
    )
}

/// Creates and prints main menu, asking for prompt
pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let _ = disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\nMain Menu
1. Settings
2. Scheduling
0. Quit"
    );
    let mut p = Readline::default()
        .title("Select an option:")
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
        "\n\n\nSettings Menu
1. General
2. Observatory
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
