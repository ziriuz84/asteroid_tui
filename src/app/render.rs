// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Ratatui drawing: shell layout, themed screens, tables.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Paragraph, Row, Table, Wrap};
use ratatui::Frame;

use crate::i18n::{self, keys};
use crate::observing_target_list::PossibleTarget;
use crate::settings::Settings;

use super::theme::{Theme, BANNER};
use super::{App, Screen, TargetDraft, TargetStep, LANGUAGES, OBJECT_TYPES, OBSERVATORY_FIELDS};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_CONTENT_WIDTH: u16 = 72;
const HEADER_HEIGHT: u16 = 3;
const FOOTER_HEIGHT: u16 = 2;

/// Draw the full frame: header, centered body, themed footer.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Min(4),
            Constraint::Length(FOOTER_HEIGHT),
        ])
        .split(area);

    draw_header(frame, chunks[0], app);
    let body = centered_area(chunks[1], MAX_CONTENT_WIDTH);

    match &app.screen {
        Screen::MainMenu => draw_main_menu(frame, body),
        Screen::SettingsMenu => draw_settings_menu(frame, body),
        Screen::GeneralSettings => draw_general_settings(frame, body, app),
        Screen::LanguageSelect { selected } => draw_language_select(frame, body, *selected),
        Screen::ObservatoryField { index, values } => {
            draw_observatory_field(frame, body, app, *index, values)
        }
        Screen::SchedulingMenu => draw_scheduling_menu(frame, body),
        Screen::WeatherTable { headers, rows, scroll } => {
            draw_weather_table(frame, body, headers, rows, *scroll)
        }
        Screen::SunMoonView { lines } => draw_sun_moon_view(frame, body, lines),
        Screen::TargetWizard { step, draft, input } => {
            draw_target_wizard(frame, body, *step, draft.as_ref(), input)
        }
        Screen::TargetTable { rows, scroll } => draw_target_table(frame, body, rows, *scroll),
        Screen::Status { message, .. } => draw_status(frame, body, message),
    }

    draw_footer(frame, chunks[2], app);
}

fn centered_area(area: Rect, max_width: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y;
    Rect::new(x, y, width, height)
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let place = app.settings.get_place();
    let line = Line::from(vec![
        Span::styled(format!(" asteroid-tui v{VERSION} "), Theme::accent_bold()),
        Span::styled(" · ", Theme::muted()),
        Span::styled(place.to_string(), Theme::muted()),
    ]);
    let p = Paragraph::new(line).alignment(Alignment::Center);
    frame.render_widget(p, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let status = if app.loading {
        let spin = Theme::spinner_frame(app.loading_tick);
        format!("{spin} {}", app.status_line)
    } else {
        app.status_line.clone()
    };

    let lines = vec![
        Line::from(Span::styled(status, Theme::accent())),
        Line::from(Span::styled(
            "9/Esc back · q quit · 0 menu · Enter confirm · j/k navigate",
            Theme::muted(),
        )),
    ];
    frame.render_widget(Paragraph::new(lines), area);
}

fn draw_main_menu(frame: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = BANNER
        .iter()
        .map(|l| Line::from(Span::styled((*l).to_string(), Theme::accent())))
        .collect();
    lines.push(Line::from(""));
    lines.push(Theme::menu_title(&i18n::t(keys::MAIN_MENU_TITLE)));
    lines.push(Theme::menu_item("1", &i18n::t(keys::SETTINGS)));
    lines.push(Theme::menu_item("2", &i18n::t(keys::SCHEDULING)));
    lines.push(Theme::menu_item("0", &i18n::t(keys::QUIT)));

    let block = Theme::block("Main Menu");
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_settings_menu(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Theme::menu_title(&i18n::t(keys::SETTINGS_MENU_TITLE)),
        Theme::menu_item("1", &i18n::t(keys::GENERAL)),
        Theme::menu_item("2", &i18n::t(keys::OBSERVATORY)),
        Theme::menu_item("9", &i18n::t(keys::BACK)),
        Theme::menu_item("0", &i18n::t(keys::QUIT)),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Theme::block("Settings")),
        area,
    );
}

fn draw_general_settings(frame: &mut Frame, area: Rect, app: &App) {
    let lines = vec![
        Theme::title_line(&format!(
            "{} — lang: {}",
            i18n::t(keys::GENERAL),
            app.settings.get_lang()
        )),
        Line::from(""),
        Theme::menu_item("1", "Language / Lingua"),
        Theme::menu_item("9", &i18n::t(keys::BACK)),
        Theme::menu_item("0", &i18n::t(keys::QUIT)),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Theme::block("General")),
        area,
    );
}

fn draw_scheduling_menu(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Theme::menu_title(&i18n::t(keys::SCHEDULING_MENU_TITLE)),
        Theme::menu_item("1", &i18n::t(keys::WEATHER_FORECAST)),
        Theme::menu_item("2", &i18n::t(keys::SUN_MOON_TIMES)),
        Theme::menu_item("3", &i18n::t(keys::OBSERVING_TARGET_LIST)),
        Theme::menu_item("9", &i18n::t(keys::BACK)),
        Theme::menu_item("0", &i18n::t(keys::QUIT)),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Theme::block("Scheduling")),
        area,
    );
}

fn draw_language_select(frame: &mut Frame, area: Rect, selected: usize) {
    let items: Vec<ListItem> = LANGUAGES
        .iter()
        .enumerate()
        .map(|(i, lang)| {
            let prefix = if i == selected { "► " } else { "  " };
            let style = if i == selected {
                Theme::highlight()
            } else {
                Style::default()
            };
            ListItem::new(format!("{prefix}{lang}")).style(style)
        })
        .collect();
    frame.render_widget(
        List::new(items).block(Theme::block("Language / Lingua")),
        area,
    );
}

fn observatory_field_label(index: usize, settings: &Settings) -> String {
    match index {
        0 => format!("Place Name ({}): ", settings.get_place()),
        1 => format!("Latitude ({}): ", settings.get_latitude()),
        2 => format!("Longitude ({}): ", settings.get_longitude()),
        3 => format!("Altitude ({}): ", settings.get_altitude()),
        4 => format!("Observatory Name ({}): ", settings.get_observatory_name()),
        5 => format!("Observer Name ({}): ", settings.get_observer_name()),
        6 => format!("MPC Code ({}): ", settings.get_mpc_code()),
        7 => format!("North Altitude ({}): ", settings.get_north_altitude()),
        8 => format!("South Altitude ({}): ", settings.get_south_altitude()),
        9 => format!("East Altitude ({}): ", settings.get_east_altitude()),
        10 => format!("West Altitude ({}): ", settings.get_west_altitude()),
        _ => String::new(),
    }
}

fn draw_observatory_field(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    index: usize,
    values: &[String],
) {
    let label = observatory_field_label(index, &app.settings);
    let current = values.get(index).map(String::as_str).unwrap_or("");
    let progress = Theme::progress_bar(index + 1, OBSERVATORY_FIELDS, 20);

    let lines = vec![
        Theme::title_line("Observatory Settings"),
        Line::from(Span::styled(progress, Theme::muted())),
        Line::from(""),
        Line::from(Span::styled(label, Theme::muted())),
        Theme::input_display(current),
        Line::from(""),
        Line::from(Span::styled(
            "Enter: next/save · Esc/9: back",
            Theme::muted(),
        )),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Theme::block("Observatory")),
        area,
    );
}

fn target_step_index(step: TargetStep) -> usize {
    match step {
        TargetStep::Year => 0,
        TargetStep::Month => 1,
        TargetStep::Day => 2,
        TargetStep::Hour => 3,
        TargetStep::Minute => 4,
        TargetStep::Duration => 5,
        TargetStep::MaxObjects => 6,
        TargetStep::MinAlt => 7,
        TargetStep::SolarElong => 8,
        TargetStep::LunarElong => 9,
        TargetStep::ObjectType => 10,
    }
}

const TARGET_WIZARD_STEPS: usize = 11;

fn target_step_title(step: TargetStep) -> &'static str {
    match step {
        TargetStep::Year => "Year (YYYY)",
        TargetStep::Month => "Month (MM)",
        TargetStep::Day => "Day (DD)",
        TargetStep::Hour => "Hour (HH, 0-23)",
        TargetStep::Minute => "Minute (MM, 0-59)",
        TargetStep::Duration => "Duration in hours",
        TargetStep::MaxObjects => "Maximum number of objects",
        TargetStep::MinAlt => "Minimum Altitude (deg)",
        TargetStep::SolarElong => "Maximum Solar elongation (deg)",
        TargetStep::LunarElong => "Maximum Lunar elongation (deg)",
        TargetStep::ObjectType => "Select object type",
    }
}

fn draw_target_wizard(
    frame: &mut Frame,
    area: Rect,
    step: TargetStep,
    draft: &TargetDraft,
    input: &str,
) {
    if step == TargetStep::ObjectType {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(4),
                Constraint::Length(2),
            ])
            .split(area);

        let hint = Paragraph::new(Line::from(Span::styled(
            "↑↓/j/k move · 1/2/3 pick · Enter: fetch MPC",
            Theme::muted(),
        )));
        frame.render_widget(hint, chunks[0]);

        let items: Vec<ListItem> = OBJECT_TYPES
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let prefix = if i == draft.object_type_index { "► " } else { "  " };
                let style = if i == draft.object_type_index {
                    Theme::highlight()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{prefix}{t} ({})", i + 1)).style(style)
            })
            .collect();
        frame.render_widget(
            List::new(items).block(Theme::block("Object type")),
            chunks[1],
        );

        let footer = Paragraph::new(Line::from(Span::styled(
            "Press Enter to fetch the target list",
            Theme::accent(),
        )));
        frame.render_widget(footer, chunks[2]);
        return;
    }

    let step_num = target_step_index(step) + 1;
    let progress = Theme::progress_bar(step_num, TARGET_WIZARD_STEPS, 20);
    let lines = vec![
        Theme::title_line("Observing Target List"),
        Line::from(Span::styled(progress, Theme::muted())),
        Line::from(""),
        Line::from(Span::styled(
            target_step_title(step).to_string(),
            Theme::muted(),
        )),
        Theme::input_display(input),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Theme::block("Target list")),
        area,
    );
}

fn draw_weather_table(
    frame: &mut Frame,
    area: Rect,
    headers: &[String],
    rows: &[Vec<String>],
    scroll: usize,
) {
    let visible = area.height.saturating_sub(4) as usize;
    let total = rows.len();
    let title = if total > visible {
        format!("Weather · row {}/{}", scroll + 1, total)
    } else {
        "Weather Forecast".to_string()
    };

    let header = Row::new(headers.iter().map(|h| h.as_str()).collect::<Vec<_>>())
        .style(Theme::table_header());
    let table_rows: Vec<Row> = rows
        .iter()
        .skip(scroll)
        .take(visible)
        .enumerate()
        .map(|(i, r)| {
            let style = if i % 2 == 1 {
                Theme::table_row_alt()
            } else {
                Style::default()
            };
            Row::new(r.iter().map(|c| c.as_str()).collect::<Vec<_>>()).style(style)
        })
        .collect();

    let table = Table::new(
        table_rows,
        headers
            .iter()
            .map(|_| Constraint::Min(8))
            .collect::<Vec<_>>(),
    )
    .header(header)
    .block(Theme::block(&title));
    frame.render_widget(table, area);
}

fn draw_target_table(frame: &mut Frame, area: Rect, rows: &[PossibleTarget], scroll: usize) {
    let headers = ["Designation", "Mag", "RA", "DEC", "Alt"];
    let visible = area.height.saturating_sub(4) as usize;
    let total = rows.len();
    let title = format!("Targets ({total}) · row {}/{}", scroll + 1, total.max(1));

    let header = Row::new(headers).style(Theme::table_header());
    let table_rows: Vec<Row> = rows
        .iter()
        .skip(scroll)
        .take(visible)
        .enumerate()
        .map(|(i, t)| {
            let style = if i % 2 == 1 {
                Theme::table_row_alt()
            } else {
                Style::default()
            };
            Row::new(vec![
                t.designation.clone(),
                t.magnitude.to_string(),
                t.ra.clone(),
                t.dec.clone(),
                t.altitude.to_string(),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        table_rows,
        [
            Constraint::Min(12),
            Constraint::Length(6),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .block(Theme::block(&title));
    frame.render_widget(table, area);
}

fn draw_sun_moon_view(frame: &mut Frame, area: Rect, lines: &[String]) {
    let parsed: Vec<Line> = lines
        .iter()
        .map(|line| {
            if let Some((key, value)) = line.split_once(':') {
                Theme::label_value(key.trim(), value.trim())
            } else {
                Line::from(Span::styled(line.clone(), Theme::accent_bold()))
            }
        })
        .collect();
    frame.render_widget(
        Paragraph::new(parsed)
            .block(Theme::block("Sun / Moon Times"))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn draw_status(frame: &mut Frame, area: Rect, message: &str) {
    let is_error = message.to_lowercase().contains("error");
    let style = if is_error {
        Theme::error()
    } else {
        Theme::success()
    };
    let lines = vec![
        Theme::title_line("Message"),
        Line::from(""),
        Line::from(Span::styled(message.to_string(), style)),
        Line::from(""),
        Line::from(Span::styled("Enter or 9: continue", Theme::muted())),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .block(Theme::block("Status"))
            .wrap(Wrap { trim: false }),
        area,
    );
}
