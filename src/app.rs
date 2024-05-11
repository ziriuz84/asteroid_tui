use crate::settings::Settings;
use crate::weather::prepare_data;
use crate::weather::ForecastResponse;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum CurrentScreen {
    MainMenu,
    SchedulingMenu,
    WeatherForecast,
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub current_screen: CurrentScreen,
    pub weather_requested: ForecastResponse,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub input: String,
}

impl Default for App {
    fn default() -> Self {
        let weather_data: ForecastResponse = prepare_data().unwrap();
        Self {
            running: true,
            counter: 0,
            current_screen: CurrentScreen::MainMenu,
            weather_requested: weather_data,
            character_index: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
