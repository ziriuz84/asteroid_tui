use promkit::{
    crossterm::{
        self, cursor, execute,
        style::Color,
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    },
    preset::readline::Readline,
    suggest::Suggest,
};

const OPTIONS_MAIN_MENU: [&str; 2] = ["1", "0"];
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

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n   Main Menu
    1. Settings
    0. Quit"
    );
    let mut p = Readline::default()
        .title("Select an option:")
        .validator(validate_main_menu_option, generate_main_menu_error_message)
        .prompt()?;
    let mut result = p.run()?;
    match result.as_str() {
        "1" => settings_menu()?,
        _ => (),
    }
    Ok(())
}

pub fn settings_menu() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode();
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    println!(
        "\n\n\n   Main Menu
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
    let mut result = p.run()?;
    match result.as_str() {
        "b" => main_menu()?,
        _ => (),
    }
    Ok(())
}
