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

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub current_screen: CurrentScreen,
    pub weather_requested: ForecastResponse,
}

impl Default for App {
    fn default() -> Self {
        let weather_data: ForecastResponse = prepare_data().unwrap();
        Self {
            running: true,
            counter: 0,
            current_screen: CurrentScreen::MainMenu,
            weather_requested: weather_data,
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

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
