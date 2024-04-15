use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::CurrentScreen;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match app.current_screen {
        CurrentScreen::MainMenu => {
            render_main_menu(app, frame);
        }
        CurrentScreen::SchedulingMenu => {
            render_scheduling_menu(app, frame);
        }
        CurrentScreen::WeatherForecast => {
            render_weather_forecast(app, frame);
        }
    }
}

fn render_main_menu(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.size(),
    )
}

fn render_scheduling_menu(app: &mut App, frame: &mut Frame) {}

fn render_weather_forecast(app: &mut App, frame: &mut Frame) {}
