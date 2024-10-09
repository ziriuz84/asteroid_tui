use promkit::{preset::readline::Readline, suggest::Suggest};

const OPTIONS: [&str; 2] = ["1", "0"];

// Funzione di validazione
fn validate_option(option: &str) -> bool {
    OPTIONS.contains(&option)
}

// Funzione per generare il messaggio di errore
fn generate_error_message(option: &str) -> String {
    format!(
        "Invalid option: {}. Please choose between {}.",
        option,
        OPTIONS.join(", ")
    )
}

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "   Main Menu
    1. Settings
    0. Quit"
    );
    let mut p = Readline::default()
        .title("Select an option:")
        .validator(validate_option, generate_error_message)
        .prompt()?;
    println!("Option selected: {:?}", p.run()?);
    Ok(())
}
