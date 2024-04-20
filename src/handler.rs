use crate::app::{App, AppResult, CurrentScreen};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('s') | KeyCode::Char('S') => match app.current_screen {
            CurrentScreen::MainMenu => app.current_screen = CurrentScreen::SchedulingMenu,
            _ => {}
        },
        KeyCode::Char('b') | KeyCode::Char('B') => match app.current_screen {
            CurrentScreen::SchedulingMenu => app.current_screen = CurrentScreen::MainMenu,
            CurrentScreen::WeatherForecast => app.current_screen = CurrentScreen::SchedulingMenu,
            _ => {}
        },
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
