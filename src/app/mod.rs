// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Full-screen Ratatui application (menus, forms, tables).

mod validation;

use std::io::{stdout, Stdout};

use anyhow::{Context, Result};
use chrono::format::StrftimeItems;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, terminal::Clear, terminal::ClearType};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Wrap};
use ratatui::Terminal;

use crate::i18n::{self, keys};
use crate::observing_target_list::{parse_whats_up_response, PossibleTarget, WhatsUpParams};
use crate::settings::{default_mpc_auth_token, General, Observatory, Settings};
use crate::sun_moon_times::SunMoonTimesResponse;
use crate::weather::ForecastResponse;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const OBJECT_TYPES: [&str; 3] = ["Asteroid", "NEO", "Comet"];
const LANGUAGES: [&str; 2] = ["en", "it"];
const OBSERVATORY_FIELDS: usize = 11;

/// Target list wizard field indices.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TargetStep {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Duration,
    MaxObjects,
    MinAlt,
    SolarElong,
    LunarElong,
    ObjectType,
}

/// Draft for What's Up parameters.
#[derive(Clone, Default)]
struct TargetDraft {
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    duration: String,
    max_objects: String,
    min_alt: String,
    solar_elong: String,
    lunar_elong: String,
    object_type_index: usize,
}

/// Application screen state.
#[derive(Clone)]
enum Screen {
    MainMenu,
    SettingsMenu,
    GeneralSettings,
    LanguageSelect { selected: usize },
    ObservatoryField { index: usize, values: Vec<String> },
    SchedulingMenu,
    WeatherTable { headers: Vec<String>, rows: Vec<Vec<String>>, scroll: usize },
    SunMoonView { lines: Vec<String> },
    TargetWizard {
        step: TargetStep,
        draft: Box<TargetDraft>,
        input: String,
    },
    TargetTable {
        rows: Vec<PossibleTarget>,
        scroll: usize,
    },
    Status { message: String, back: Box<Screen> },
}

/// Main application state.
struct App {
    screen: Screen,
    should_quit: bool,
    status_line: String,
    settings: Settings,
}

impl App {
    fn new() -> Result<Self> {
        let settings = Settings::new().context("Failed to load settings")?;
        Ok(Self {
            screen: Screen::MainMenu,
            should_quit: false,
            status_line: i18n::t(keys::SELECT_OPTION),
            settings,
        })
    }

    fn reload_settings(&mut self) -> Result<()> {
        self.settings = Settings::new().context("Failed to load settings")?;
        Ok(())
    }
}

/// Runs the full-screen TUI until the user quits.
pub fn run() -> Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;
    let mut app = App::new()?;
    let result = run_loop(&mut terminal, &mut app);
    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Clear(ClearType::All)).ok();
    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app))?;
        if app.should_quit {
            break;
        }
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                handle_key(app, key)?;
            }
        }
    }
    Ok(())
}

fn render(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    match &app.screen {
        Screen::MainMenu => draw_main_menu(frame, chunks[0], app),
        Screen::SettingsMenu => draw_settings_menu(frame, chunks[0], app),
        Screen::GeneralSettings => draw_general_settings(frame, chunks[0], app),
        Screen::LanguageSelect { selected } => {
            draw_language_select(frame, chunks[0], *selected)
        }
        Screen::ObservatoryField { index, values } => {
            draw_observatory_field(frame, chunks[0], app, *index, values)
        }
        Screen::SchedulingMenu => draw_scheduling_menu(frame, chunks[0], app),
        Screen::WeatherTable { headers, rows, scroll } => {
            draw_weather_table(frame, chunks[0], headers, rows, *scroll)
        }
        Screen::SunMoonView { lines } => draw_text_block(frame, chunks[0], "Sun / Moon Times", lines),
        Screen::TargetWizard { step, draft, input } => {
            draw_target_wizard(frame, chunks[0], *step, draft.as_ref(), input)
        }
        Screen::TargetTable { rows, scroll } => draw_target_table(frame, chunks[0], rows, *scroll),
        Screen::Status { message, .. } => draw_status(frame, chunks[0], message),
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::raw(&app.status_line),
        Span::raw("  |  "),
        Span::raw("9/Esc: back  q: quit  0: menu option  Enter: confirm  j/k: navigate"),
    ]));
    frame.render_widget(footer, chunks[1]);
}

fn draw_main_menu(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let text = format!(
        "asteroid-tui v{VERSION}\n\n{}\n\n1. {}\n2. {}\n0. {}",
        i18n::t(keys::MAIN_MENU_TITLE),
        i18n::t(keys::SETTINGS),
        i18n::t(keys::SCHEDULING),
        i18n::t(keys::QUIT),
    );
    let block = Block::default().title(" asteroid-tui ").borders(Borders::ALL);
    let p = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(p, area);
    let _ = app;
}

fn draw_settings_menu(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let text = format!(
        "{}\n\n1. {}\n2. {}\n9. {}\n0. {}",
        i18n::t(keys::SETTINGS_MENU_TITLE),
        i18n::t(keys::GENERAL),
        i18n::t(keys::OBSERVATORY),
        i18n::t(keys::BACK),
        i18n::t(keys::QUIT),
    );
    let block = Block::default().title(" Settings ").borders(Borders::ALL);
    frame.render_widget(Paragraph::new(text).block(block), area);
    let _ = app;
}

fn draw_general_settings(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let text = format!(
        "{} (lang: {})\n\n1. Language\n9. {}\n0. {}",
        i18n::t(keys::GENERAL),
        app.settings.get_lang(),
        i18n::t(keys::BACK),
        i18n::t(keys::QUIT),
    );
    let block = Block::default().title(" General ").borders(Borders::ALL);
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn draw_language_select(frame: &mut ratatui::Frame, area: Rect, selected: usize) {
    let items: Vec<ListItem> = LANGUAGES
        .iter()
        .enumerate()
        .map(|(i, lang)| {
            let style = if i == selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(*lang).style(style)
        })
        .collect();
    let block = Block::default()
        .title(" Language / Lingua ")
        .borders(Borders::ALL);
    frame.render_widget(List::new(items).block(block), area);
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
        7 => format!(
            "North Altitude ({}): ",
            settings.get_north_altitude()
        ),
        8 => format!(
            "South Altitude ({}): ",
            settings.get_south_altitude()
        ),
        9 => format!("East Altitude ({}): ", settings.get_east_altitude()),
        10 => format!("West Altitude ({}): ", settings.get_west_altitude()),
        _ => String::new(),
    }
}

fn draw_observatory_field(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &App,
    index: usize,
    values: &[String],
) {
    let label = observatory_field_label(index, &app.settings);
    let current = values.get(index).map(String::as_str).unwrap_or("");
    let progress = format!("Field {} of {}", index + 1, OBSERVATORY_FIELDS);
    let text = format!(
        "Observatory Settings\n{progress}\n\n{label}\n[{current}]\n\nEnter: next/save  Esc: cancel",
    );
    let block = Block::default().title(" Observatory ").borders(Borders::ALL);
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn draw_scheduling_menu(frame: &mut ratatui::Frame, area: Rect, _app: &App) {
    let text = format!(
        "{}\n\n1. {}\n2. {}\n3. {}\n9. {}\n0. {}",
        i18n::t(keys::SCHEDULING_MENU_TITLE),
        i18n::t(keys::WEATHER_FORECAST),
        i18n::t(keys::SUN_MOON_TIMES),
        i18n::t(keys::OBSERVING_TARGET_LIST),
        i18n::t(keys::BACK),
        i18n::t(keys::QUIT),
    );
    let block = Block::default().title(" Scheduling ").borders(Borders::ALL);
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn draw_weather_table(
    frame: &mut ratatui::Frame,
    area: Rect,
    headers: &[String],
    rows: &[Vec<String>],
    scroll: usize,
) {
    let header = Row::new(headers.iter().map(|h| h.as_str()).collect::<Vec<_>>())
        .style(Style::default().add_modifier(Modifier::BOLD));
    let table_rows: Vec<Row> = rows
        .iter()
        .skip(scroll)
        .take(area.height.saturating_sub(4) as usize)
        .map(|r| Row::new(r.iter().map(|c| c.as_str()).collect::<Vec<_>>()))
        .collect();
    let block = Block::default()
        .title(" Weather Forecast ")
        .borders(Borders::ALL);
    let table = Table::new(table_rows, headers.iter().map(|_| Constraint::Length(10)))
        .header(header)
        .block(block);
    frame.render_widget(table, area);
}

fn draw_target_table(
    frame: &mut ratatui::Frame,
    area: Rect,
    rows: &[PossibleTarget],
    scroll: usize,
) {
    let headers = ["Designation", "Mag", "RA", "DEC", "Alt"];
    let header = Row::new(headers).style(Style::default().add_modifier(Modifier::BOLD));
    let table_rows: Vec<Row> = rows
        .iter()
        .skip(scroll)
        .take(area.height.saturating_sub(4) as usize)
        .map(|t| {
            Row::new(vec![
                t.designation.clone(),
                t.magnitude.to_string(),
                t.ra.clone(),
                t.dec.clone(),
                t.altitude.to_string(),
            ])
        })
        .collect();
    let block = Block::default()
        .title(format!(" Targets ({}) ", rows.len()))
        .borders(Borders::ALL);
    let table = Table::new(
        table_rows,
        [
            Constraint::Length(14),
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .block(block);
    frame.render_widget(table, area);
}

fn draw_text_block(frame: &mut ratatui::Frame, area: Rect, title: &str, lines: &[String]) {
    let text = lines.join("\n");
    let block = Block::default().title(format!(" {title} ")).borders(Borders::ALL);
    frame.render_widget(
        Paragraph::new(text).block(block).wrap(Wrap { trim: false }),
        area,
    );
}

fn target_step_title(step: TargetStep) -> &'static str {
    match step {
        TargetStep::Year => "Year (YYYY):",
        TargetStep::Month => "Month (MM):",
        TargetStep::Day => "Day (DD):",
        TargetStep::Hour => "Hour (HH, 0-23):",
        TargetStep::Minute => "Minute (MM, 0-59):",
        TargetStep::Duration => "Duration in hours:",
        TargetStep::MaxObjects => "Maximum number of objects:",
        TargetStep::MinAlt => "Minimum Altitude (deg):",
        TargetStep::SolarElong => "Maximum Solar elongation (deg):",
        TargetStep::LunarElong => "Maximum Lunar elongation (deg):",
        TargetStep::ObjectType => "Select object type (j/k, Enter):",
    }
}

fn draw_target_wizard(
    frame: &mut ratatui::Frame,
    area: Rect,
    step: TargetStep,
    draft: &TargetDraft,
    input: &str,
) {
    if step == TargetStep::ObjectType {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(4),
            ])
            .split(area);
        let hint = Paragraph::new(
            "Select object type / Seleziona tipo oggetto\n\
             Keys: ↑↓ or j/k move | 1 Asteroid | 2 NEO | 3 Comet | Enter confirm",
        );
        frame.render_widget(hint, chunks[0]);
        let items: Vec<ListItem> = OBJECT_TYPES
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let prefix = if i == draft.object_type_index { "► " } else { "  " };
                let style = if i == draft.object_type_index {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{prefix}{} ({})", t, i + 1)).style(style)
            })
            .collect();
        let block = Block::default()
            .title(" Object type ")
            .borders(Borders::ALL);
        frame.render_widget(List::new(items).block(block), chunks[1]);
        let footer = Paragraph::new("Press Enter to fetch the target list from MPC");
        frame.render_widget(footer, chunks[2]);
        return;
    }
    let text = format!(
        "Observing Target List\n\n{}\n\n[{input}]",
        target_step_title(step),
    );
    let block = Block::default()
        .title(" Observing Target List ")
        .borders(Borders::ALL);
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn draw_status(frame: &mut ratatui::Frame, area: Rect, message: &str) {
    let block = Block::default().title(" Message ").borders(Borders::ALL);
    frame.render_widget(
        Paragraph::new(message.to_string())
            .block(block)
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return Ok(());
    }

    match &app.screen.clone() {
        Screen::MainMenu => handle_main_menu(app, key),
        Screen::SettingsMenu => handle_settings_menu(app, key),
        Screen::GeneralSettings => handle_general_settings(app, key),
        Screen::LanguageSelect { selected } => handle_language_select(app, key, *selected),
        Screen::ObservatoryField { index, values } => {
            handle_observatory_field(app, key, *index, values.clone())
        }
        Screen::SchedulingMenu => handle_scheduling_menu(app, key),
        Screen::WeatherTable { scroll, .. } => handle_scrollable_back(app, key, *scroll, true),
        Screen::SunMoonView { .. } => handle_simple_back(app, key, Screen::SchedulingMenu),
        Screen::TargetWizard { step, draft, input } => {
            handle_target_wizard(app, key, *step, draft.as_ref().clone(), input.clone())
        }
        Screen::TargetTable { scroll, .. } => {
            handle_scrollable_back(app, key, *scroll, false)
        }
        Screen::Status { back, .. } => handle_status(app, key, back.as_ref().clone()),
    }
}

fn handle_main_menu(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('1') => {
            app.screen = Screen::SettingsMenu;
            app.status_line = i18n::t(keys::SELECT_OPTION);
        }
        KeyCode::Char('2') => {
            app.screen = Screen::SchedulingMenu;
            app.status_line = i18n::t(keys::SELECT_OPTION);
        }
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => app.status_line = invalid_option_hint(&["1", "2", "0"]),
    }
    Ok(())
}

fn handle_settings_menu(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('1') => app.screen = Screen::GeneralSettings,
        KeyCode::Char('2') => start_observatory_wizard(app)?,
        KeyCode::Char('9') | KeyCode::Esc => app.screen = Screen::MainMenu,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => app.status_line = invalid_option_hint(&["1", "2", "9", "0"]),
    }
    Ok(())
}

fn handle_general_settings(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('1') => {
            let idx = LANGUAGES
                .iter()
                .position(|l| *l == app.settings.get_lang())
                .unwrap_or(0);
            app.screen = Screen::LanguageSelect { selected: idx };
        }
        KeyCode::Char('9') | KeyCode::Esc => app.screen = Screen::SettingsMenu,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => app.status_line = invalid_option_hint(&["1", "9", "0"]),
    }
    Ok(())
}

fn handle_language_select(app: &mut App, key: KeyEvent, selected: usize) -> Result<()> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let s = selected.saturating_sub(1);
            app.screen = Screen::LanguageSelect { selected: s };
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let s = (selected + 1).min(LANGUAGES.len() - 1);
            app.screen = Screen::LanguageSelect { selected: s };
        }
        KeyCode::Enter => {
            let lang = LANGUAGES[selected].to_string();
            app.settings
                .set_lang(lang)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            app.reload_settings()?;
            app.screen = Screen::Status {
                message: "Language saved / Lingua salvata".to_string(),
                back: Box::new(Screen::GeneralSettings),
            };
        }
        KeyCode::Esc | KeyCode::Char('9') => app.screen = Screen::GeneralSettings,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
    Ok(())
}

fn start_observatory_wizard(app: &mut App) -> Result<()> {
    app.reload_settings()?;
    app.screen = Screen::ObservatoryField {
        index: 0,
        values: vec![String::new(); OBSERVATORY_FIELDS],
    };
    app.status_line =
        "Type value, Enter next (empty=keep) | 9/Esc back | q quit".to_string();
    Ok(())
}

fn handle_observatory_field(
    app: &mut App,
    key: KeyEvent,
    index: usize,
    mut values: Vec<String>,
) -> Result<()> {
    let buf = values.get_mut(index).unwrap();
    match key.code {
        KeyCode::Esc | KeyCode::Char('9') => app.screen = Screen::SettingsMenu,
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Enter => {
            if index == 1 && !buf.is_empty() && !validation::validate_latitude(buf) {
                app.status_line = "Latitude must be -90..90".to_string();
                return Ok(());
            }
            if index == 2 && !buf.is_empty() && !validation::validate_longitude(buf) {
                app.status_line = "Longitude must be -180..180".to_string();
                return Ok(());
            }
            if index + 1 >= OBSERVATORY_FIELDS {
                save_observatory(app, &values)?;
                app.screen = Screen::Status {
                    message: "Settings saved / Impostazioni salvate".to_string(),
                    back: Box::new(Screen::SettingsMenu),
                };
            } else {
                app.screen = Screen::ObservatoryField {
                    index: index + 1,
                    values,
                };
            }
        }
        KeyCode::Backspace => {
            buf.pop();
        }
        KeyCode::Char(c) => {
            buf.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn save_observatory(app: &mut App, values: &[String]) -> Result<()> {
    let new_settings = settings_from_observatory_values(&app.settings, values)?;
    app.settings
        .set_settings(new_settings)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    app.reload_settings()?;
    Ok(())
}

fn settings_from_observatory_values(
    actual: &Settings,
    value: &[String],
) -> Result<Settings> {
    let general = General {
        lang: actual.general.lang.clone(),
        mpc_auth_token: default_mpc_auth_token(),
    };
    let observatory = Observatory {
        place: if value[0].is_empty() {
            actual.get_place().to_string()
        } else {
            value[0].clone()
        },
        latitude: if value[1].is_empty() {
            *actual.get_latitude()
        } else {
            value[1].parse::<f32>()?
        },
        longitude: if value[2].is_empty() {
            *actual.get_longitude()
        } else {
            value[2].parse::<f32>()?
        },
        altitude: if value[3].is_empty() {
            *actual.get_altitude()
        } else {
            value[3].parse::<f32>()?
        },
        observatory_name: if value[4].is_empty() {
            actual.get_observatory_name().to_string()
        } else {
            value[4].clone()
        },
        observer_name: if value[5].is_empty() {
            actual.get_observer_name().to_string()
        } else {
            value[5].clone()
        },
        mpc_code: if value[6].is_empty() {
            actual.get_mpc_code().to_string()
        } else {
            value[6].clone()
        },
        north_altitude: if value[7].is_empty() {
            *actual.get_north_altitude()
        } else {
            value[7].parse::<i32>()?
        },
        south_altitude: if value[8].is_empty() {
            *actual.get_south_altitude()
        } else {
            value[8].parse::<i32>()?
        },
        east_altitude: if value[9].is_empty() {
            *actual.get_east_altitude()
        } else {
            value[9].parse::<i32>()?
        },
        west_altitude: if value[10].is_empty() {
            *actual.get_west_altitude()
        } else {
            value[10].parse::<i32>()?
        },
    };
    Ok(Settings {
        general,
        observatory,
    })
}

fn handle_scheduling_menu(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('1') => load_weather(app)?,
        KeyCode::Char('2') => load_sun_moon(app)?,
        KeyCode::Char('3') => start_target_wizard(app),
        KeyCode::Char('9') | KeyCode::Esc => app.screen = Screen::MainMenu,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => app.status_line = invalid_option_hint(&["1", "2", "3", "9", "0"]),
    }
    Ok(())
}

fn load_weather(app: &mut App) -> Result<()> {
    app.status_line = "Fetching weather...".to_string();
    let data = crate::weather::prepare_data().map_err(|e| {
        app.screen = Screen::Status {
            message: format!("Weather error: {e}"),
            back: Box::new(Screen::SchedulingMenu),
        };
        e
    })?;
    let (headers, rows) = build_weather_rows(&data);
    app.screen = Screen::WeatherTable {
        headers,
        rows,
        scroll: 0,
    };
    app.status_line = "j/k: scroll  9/Esc: back".to_string();
    Ok(())
}

fn load_sun_moon(app: &mut App) -> Result<()> {
    app.status_line = "Fetching sun/moon times...".to_string();
    let data: SunMoonTimesResponse = crate::sun_moon_times::prepare_data().map_err(|e| {
        app.screen = Screen::Status {
            message: format!("Sun/moon error: {e}"),
            back: Box::new(Screen::SchedulingMenu),
        };
        e
    })?;
    let r = &data.results;
    let lines = vec![
        format!("All times are {}", data.tzid),
        format!("Sunrise: {}", r.sunrise),
        format!("Sunset: {}", r.sunset),
        format!("Solar noon: {}", r.solar_noon),
        format!("Day length: {}", r.day_length),
        format!("Civil twilight begin: {}", r.civil_twilight_begin),
        format!("Civil twilight end: {}", r.civil_twilight_end),
        format!("Nautical twilight begin: {}", r.nautical_twilight_begin),
        format!("Nautical twilight end: {}", r.nautical_twilight_end),
        format!(
            "Astronomical twilight begin: {}",
            r.astronomical_twilight_begin
        ),
        format!("Astronomical twilight end: {}", r.astronomical_twilight_end),
    ];
    app.screen = Screen::SunMoonView { lines };
    app.status_line = "9/Esc: back".to_string();
    Ok(())
}

fn start_target_wizard(app: &mut App) {
    app.screen = Screen::TargetWizard {
        step: TargetStep::Year,
        draft: Box::new(TargetDraft::default()),
        input: String::new(),
    };
    app.status_line =
        "Type value, Enter confirm | 9/Esc back | q quit (0 is a digit)".to_string();
}

fn handle_target_wizard(
    app: &mut App,
    key: KeyEvent,
    step: TargetStep,
    mut draft: TargetDraft,
    mut input: String,
) -> Result<()> {
    if step == TargetStep::ObjectType {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                draft.object_type_index = draft.object_type_index.saturating_sub(1);
                app.status_line = format!(
                    "Selected: {} — press Enter to fetch",
                    OBJECT_TYPES[draft.object_type_index]
                );
            }
            KeyCode::Down | KeyCode::Char('j') => {
                draft.object_type_index =
                    (draft.object_type_index + 1).min(OBJECT_TYPES.len() - 1);
                app.status_line = format!(
                    "Selected: {} — press Enter to fetch",
                    OBJECT_TYPES[draft.object_type_index]
                );
            }
            KeyCode::Char('1') => {
                draft.object_type_index = 0;
                app.status_line = "Selected: Asteroid — press Enter to fetch".to_string();
            }
            KeyCode::Char('2') => {
                draft.object_type_index = 1;
                app.status_line = "Selected: NEO — press Enter to fetch".to_string();
            }
            KeyCode::Char('3') => {
                draft.object_type_index = 2;
                app.status_line = "Selected: Comet — press Enter to fetch".to_string();
            }
            KeyCode::Enter => {
                fetch_target_list(app, &draft)?;
                // fetch_target_list sets app.screen (table, status, or error); do not overwrite
                return Ok(());
            }
            KeyCode::Esc | KeyCode::Char('9') => {
                app.screen = Screen::SchedulingMenu;
                return Ok(());
            }
            KeyCode::Char('q') => {
                app.should_quit = true;
                return Ok(());
            }
            _ => {
                app.status_line =
                    "↑↓/j/k: move | 1/2/3: pick | Enter: fetch MPC list | 9: back".to_string();
            }
        }
        app.screen = Screen::TargetWizard {
            step,
            draft: Box::new(draft),
            input,
        };
        return Ok(());
    }

    match key.code {
        KeyCode::Esc | KeyCode::Char('9') => app.screen = Screen::SchedulingMenu,
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Backspace => {
            input.pop();
        }
        KeyCode::Char(c) => {
            input.push(c);
        }
        KeyCode::Enter => {
            if !validate_target_step(step, &input, &draft) {
                app.status_line = target_validation_message(step);
                app.screen = Screen::TargetWizard {
                    step,
                    draft: Box::new(draft),
                    input,
                };
                return Ok(());
            }
            apply_target_step(&mut draft, step, &input);
            let next = next_target_step(step);
            input.clear();
            if let Some(next_step) = next {
                app.screen = Screen::TargetWizard {
                    step: next_step,
                    draft: Box::new(draft),
                    input,
                };
            } else {
                app.screen = Screen::TargetWizard {
                    step: TargetStep::ObjectType,
                    draft: Box::new(draft),
                    input,
                };
                app.status_line =
                    "↑↓/j/k: move | 1/2/3: pick | Enter: fetch MPC list | 9: back".to_string();
            }
            return Ok(());
        }
        _ => {}
    }
    app.screen = Screen::TargetWizard {
        step,
        draft: Box::new(draft),
        input,
    };
    Ok(())
}

fn validate_target_step(step: TargetStep, input: &str, draft: &TargetDraft) -> bool {
    match step {
        TargetStep::Year => validation::validate_year(input),
        TargetStep::Month => validation::validate_month(input),
        TargetStep::Day => {
            validation::validate_day(input)
                && draft.year.parse::<u32>().ok().zip(draft.month.parse::<u32>().ok())
                    .map(|(y, m)| {
                        input
                            .parse::<u32>()
                            .map(|d| validation::validate_date(y, m, d))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
        }
        TargetStep::Hour => validation::validate_hour(input),
        TargetStep::Minute => validation::validate_minute(input),
        TargetStep::Duration
        | TargetStep::MaxObjects
        | TargetStep::MinAlt
        | TargetStep::SolarElong
        | TargetStep::LunarElong => validation::validate_positive_integer(input),
        TargetStep::ObjectType => true,
    }
}

fn target_validation_message(step: TargetStep) -> String {
    match step {
        TargetStep::Year => "Invalid year (1900-2200)".to_string(),
        TargetStep::Month => "Invalid month (1-12)".to_string(),
        TargetStep::Day => "Invalid day for month/year".to_string(),
        TargetStep::Hour => "Invalid hour (0-23)".to_string(),
        TargetStep::Minute => "Invalid minute (0-59)".to_string(),
        _ => "Invalid positive number".to_string(),
    }
}

fn apply_target_step(draft: &mut TargetDraft, step: TargetStep, input: &str) {
    match step {
        TargetStep::Year => draft.year = input.to_string(),
        TargetStep::Month => draft.month = input.to_string(),
        TargetStep::Day => draft.day = input.to_string(),
        TargetStep::Hour => draft.hour = input.to_string(),
        TargetStep::Minute => draft.minute = input.to_string(),
        TargetStep::Duration => draft.duration = input.to_string(),
        TargetStep::MaxObjects => draft.max_objects = input.to_string(),
        TargetStep::MinAlt => draft.min_alt = input.to_string(),
        TargetStep::SolarElong => draft.solar_elong = input.to_string(),
        TargetStep::LunarElong => draft.lunar_elong = input.to_string(),
        TargetStep::ObjectType => {}
    }
}

fn next_target_step(step: TargetStep) -> Option<TargetStep> {
    match step {
        TargetStep::Year => Some(TargetStep::Month),
        TargetStep::Month => Some(TargetStep::Day),
        TargetStep::Day => Some(TargetStep::Hour),
        TargetStep::Hour => Some(TargetStep::Minute),
        TargetStep::Minute => Some(TargetStep::Duration),
        TargetStep::Duration => Some(TargetStep::MaxObjects),
        TargetStep::MaxObjects => Some(TargetStep::MinAlt),
        TargetStep::MinAlt => Some(TargetStep::SolarElong),
        TargetStep::SolarElong => Some(TargetStep::LunarElong),
        TargetStep::LunarElong => Some(TargetStep::ObjectType),
        TargetStep::ObjectType => None,
    }
}

fn fetch_target_list(app: &mut App, draft: &TargetDraft) -> Result<()> {
    let object_type = OBJECT_TYPES[draft.object_type_index];
    let params = WhatsUpParams {
        year: draft.year.clone(),
        month: draft.month.clone(),
        day: draft.day.clone(),
        hour: draft.hour.clone(),
        minute: draft.minute.clone(),
        duration: draft.duration.clone(),
        max_objects: draft.max_objects.clone(),
        min_alt: draft.min_alt.clone(),
        solar_elong: draft.solar_elong.clone(),
        lunar_elong: draft.lunar_elong.clone(),
        object_type: validation::map_object_type_to_code(object_type).to_string(),
    };
    app.status_line = "Fetching targets...".to_string();
    match parse_whats_up_response(&params) {
        Ok(data) if data.is_empty() => {
            app.screen = Screen::Status {
                message: "No visible objects / Nessun oggetto visibile".to_string(),
                back: Box::new(Screen::SchedulingMenu),
            };
        }
        Ok(data) => {
            app.screen = Screen::TargetTable { rows: data, scroll: 0 };
            app.status_line = "j/k: scroll  9/Esc: back".to_string();
        }
        Err(e) => {
            app.screen = Screen::Status {
                message: format!("Target list error: {e}"),
                back: Box::new(Screen::SchedulingMenu),
            };
        }
    }
    Ok(())
}

fn handle_scrollable_back(
    app: &mut App,
    key: KeyEvent,
    scroll: usize,
    from_weather: bool,
) -> Result<()> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Screen::WeatherTable { scroll: s, .. }
            | Screen::TargetTable { scroll: s, .. } = &mut app.screen
            {
                *s = scroll.saturating_sub(1);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Screen::WeatherTable { scroll: s, .. }
            | Screen::TargetTable { scroll: s, .. } = &mut app.screen
            {
                *s = scroll.saturating_add(1);
            }
        }
        KeyCode::Esc | KeyCode::Char('9') => app.screen = Screen::SchedulingMenu,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => {
            let _ = from_weather;
        }
    }
    Ok(())
}

fn handle_simple_back(app: &mut App, key: KeyEvent, back: Screen) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('9') => app.screen = back,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
    Ok(())
}

fn handle_status(app: &mut App, key: KeyEvent, back: Screen) -> Result<()> {
    match key.code {
        KeyCode::Enter | KeyCode::Esc | KeyCode::Char('9') => app.screen = back,
        KeyCode::Char('0') | KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
    Ok(())
}

fn invalid_option_hint(options: &[&str]) -> String {
    format!(
        "{}: choose {}",
        i18n::t(keys::INVALID_OPTION),
        options.join(", ")
    )
}

fn parse_input(input: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_dt = NaiveDateTime::parse_from_str(input, "%Y%m%d%H%M")?;
    Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc))
}

fn add_hours(dt: DateTime<Utc>, hours: u32) -> DateTime<Utc> {
    dt + Duration::hours(hours.into())
}

fn format_output(dt: DateTime<Utc>) -> String {
    let items = StrftimeItems::new("%a %H");
    dt.format_with_items(items).to_string()
}

fn build_weather_rows(data: &ForecastResponse) -> (Vec<String>, Vec<Vec<String>>) {
    let headers = vec![
        "Time".into(),
        "Clouds".into(),
        "Seeing".into(),
        "Transp".into(),
        "Instab".into(),
        "RH2m".into(),
        "Wind".into(),
        "T".into(),
        "Prec".into(),
    ];
    let timezero = format!("{}00", data.init);
    let mut rows = Vec::new();
    for item in &data.dataseries {
        let time = match parse_input(timezero.as_str()) {
            Ok(result) => format_output(add_hours(result, item.timepoint as u32)),
            Err(e) => format!("parse err: {e}"),
        };
        rows.push(vec![
            time,
            item.cloud_cover.to_str().to_string(),
            item.seeing.to_str().to_string(),
            item.transparency.to_str().to_string(),
            item.lifted_index.to_str().to_string(),
            item.rh2m.to_str().to_string(),
            format!(
                "{} at {}",
                item.wind10m.direction,
                item.wind10m.speed.to_str()
            ),
            item.temp2m.to_string(),
            item.prec_type.clone(),
        ]);
    }
    (headers, rows)
}
