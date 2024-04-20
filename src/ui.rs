use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use ratatui::prelude::*;

use crate::app::App;
use crate::app::CurrentScreen;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());
    match app.current_screen {
        CurrentScreen::MainMenu => {
            render_main_menu(app, frame, layout);
        }
        CurrentScreen::SchedulingMenu => {
            render_scheduling_menu(app, frame, layout);
        }
        CurrentScreen::WeatherForecast => {
            render_weather_forecast(app, frame);
        }
    }
}

/// Centers a rect inside an area
///
/// * `r`: The area where to insert resulting rect
/// * `percent_x`: width of resulting rect in percent of `r.width()`
/// * `percent_y`: height of resulting rect in percent of `r.height()`
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_main_menu(
    app: &mut App,
    frame: &mut Frame,
    layout: std::rc::Rc<[ratatui::layout::Rect]>,
) {
    frame.render_widget(
        Paragraph::new("AsteroidTUI")
            .block(
                Block::bordered()
                    //.title("Template")
                    //.title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Red).bg(Color::Black))
            .centered(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("")
            .block(Block::default())
            .style(Style::default().bg(Color::Black)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(
            "Main Menu\n\
        \n\n\
        c - Configuration\n\
        s - Scheduling\n\
        q - quit",
        )
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .centered(),
        layout[2],
    );
    frame.render_widget(
        Paragraph::new("Press q or Ctrl+C to quit")
            .block(Block::bordered().border_type(BorderType::Rounded))
            .style(Style::default().fg(Color::Red).bg(Color::Black))
            .centered(),
        layout[3],
    );
}

fn render_scheduling_menu(
    app: &mut App,
    frame: &mut Frame,
    layout: std::rc::Rc<[ratatui::layout::Rect]>,
) {
    frame.render_widget(
        Paragraph::new("AsteroidTUI")
            .block(
                Block::bordered()
                    //.title("Template")
                    //.title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Red).bg(Color::Black))
            .centered(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("")
            .block(Block::default())
            .style(Style::default().bg(Color::Black)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(
            "Scheduling Menu\n\
        \n\n\
        w - Weather Forecast\n\
        b - Back to Main Menu\n\
        q - Quit",
        )
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .centered(),
        layout[2],
    );
    frame.render_widget(
        Paragraph::new("Press q or Ctrl+C to quit")
            .block(Block::bordered().border_type(BorderType::Rounded))
            .style(Style::default().fg(Color::Red).bg(Color::Black))
            .centered(),
        layout[3],
    );
}

fn render_weather_forecast(app: &mut App, frame: &mut Frame) {}
