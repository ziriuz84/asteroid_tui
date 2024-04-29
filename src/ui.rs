use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Cell, Paragraph, Row, Table},
    Frame,
};
use serde::{Deserialize, Serialize};
use serde_json::{Error, Result};

use ratatui::prelude::*;

use crate::app::App;
use crate::app::CurrentScreen;
use crate::weather::prepare_data;
use crate::weather::ForecastResponse;

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
            render_weather_forecast(app, frame, layout);
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

fn create_table(data: &ForecastResponse) -> Table {
    // Create the table header
    let header = vec![
        "Timepoint",
        "Cloud Cover",
        "Seeing",
        "Transparency",
        "Lifted Index",
        "RH2m",
        "Wind10m",
        "Temp2m",
        "Prec Type",
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(Style::default().add_modifier(Modifier::BOLD));

    // Create table rows
    let rows = data
        .dataseries
        .iter()
        .map(|forecast| {
            Row::new(vec![
                Cell::from(forecast.timepoint.to_string()),
                Cell::from(forecast.cloud_cover.to_string()), // Assuming CloudCover, Seeing, Transparency, Wind10m have a to_string() method
                Cell::from(forecast.seeing.to_string()),
                Cell::from(forecast.transparency.to_string()),
                Cell::from(forecast.lifted_index.to_string()),
                Cell::from(forecast.rh2m.to_string()),
                Cell::from(forecast.wind10m.direction.to_string()),
                Cell::from(forecast.wind10m.speed.to_string()),
                Cell::from(forecast.temp2m.to_string()),
                Cell::from(forecast.prec_type.clone()),
            ])
        })
        .collect::<Vec<Row>>();
    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
    ];

    // Configure the table
    Table::new(rows, widths).header(header)
}

fn render_weather_forecast(
    app: &mut App,
    frame: &mut Frame,
    layout: std::rc::Rc<[ratatui::layout::Rect]>,
) {
    let header = Row::new(vec![
        Cell::from("Time"),
        Cell::from("Cloud Cover"),
        Cell::from("Seeing"),
        Cell::from("Transparency"),
        Cell::from("Lifted Index"),
        Cell::from("RH (2m)"),
        Cell::from("Wind Dir (10m)"),
        Cell::from("Wind Speed (10m)"),
        Cell::from("Temp (2m)"),
        Cell::from("Prec Type"),
    ]);
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
        create_table(&app.weather_requested)
            .header(header)
            .style(Style::default().bg(Color::Black).fg(Color::Red)),
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
