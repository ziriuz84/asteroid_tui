// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Full-screen Ratatui application (menus, forms, tables).

mod render;
mod theme;
mod validation;

use std::io::{stdout, Stdout};

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, terminal::Clear, terminal::ClearType};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::i18n::{self, keys};
use crate::observing_target_list::{parse_whats_up_response, WhatsUpParams};
use crate::settings::Settings;
use crate::sun_moon_times::SunMoonTimesResponse;

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
    WeatherTable {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        scroll: usize,
    },
    SunMoonView { lines: Vec<String> },
    TargetWizard {
        step: TargetStep,
        draft: Box<TargetDraft>,
        input: String,
    },
    TargetTable {
        rows: Vec<crate::observing_target_list::PossibleTarget>,
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
    loading: bool,
    loading_tick: u8,
}

impl App {
    fn new() -> Result<Self> {
        let settings = Settings::new().context("Failed to load settings")?;
        Ok(Self {
            screen: Screen::MainMenu,
            should_quit: false,
            status_line: i18n::t(keys::SELECT_OPTION),
            settings,
            loading: false,
            loading_tick: 0,
        })
    }

    fn reload_settings(&mut self) -> Result<()> {
        self.settings = Settings::new().context("Failed to load settings")?;
        Ok(())
    }

    fn set_loading(&mut self, message: &str) {
        self.loading = true;
        self.loading_tick = 0;
        self.status_line = message.to_string();
    }

    fn clear_loading(&mut self) {
        self.loading = false;
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
        if app.loading {
            app.loading_tick = app.loading_tick.wrapping_add(1);
        }
        terminal.draw(|f| render::render(f, app))?;
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
        Screen::TargetTable { scroll, .. } => handle_scrollable_back(app, key, *scroll, false),
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
    app.status_line = "Type value, Enter next (empty=keep) | 9/Esc back | q quit".to_string();
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
    let new_settings = app
        .settings
        .merge_observatory_form(values)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    app.settings
        .set_settings(new_settings)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    app.reload_settings()?;
    Ok(())
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
    app.set_loading("Fetching weather...");
    let data = crate::weather::prepare_data().map_err(|e| {
        app.clear_loading();
        app.screen = Screen::Status {
            message: format!("Weather error: {e}"),
            back: Box::new(Screen::SchedulingMenu),
        };
        e
    })?;
    app.clear_loading();
    let (headers, rows) = crate::weather::forecast_table_rows(&data);
    app.screen = Screen::WeatherTable {
        headers,
        rows,
        scroll: 0,
    };
    app.status_line = "j/k: scroll  9/Esc: back".to_string();
    Ok(())
}

fn load_sun_moon(app: &mut App) -> Result<()> {
    app.set_loading("Fetching sun/moon times...");
    let data: SunMoonTimesResponse = crate::sun_moon_times::prepare_data().map_err(|e| {
        app.clear_loading();
        app.screen = Screen::Status {
            message: format!("Sun/moon error: {e}"),
            back: Box::new(Screen::SchedulingMenu),
        };
        e
    })?;
    app.clear_loading();
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
    app.status_line = "Type value, Enter confirm | 9/Esc back | q quit (0 is a digit)".to_string();
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
        TargetStep::Day => validation::validate_wizard_day(&draft.year, &draft.month, input),
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
    app.set_loading("Fetching targets from MPC...");
    match parse_whats_up_response(&params) {
        Ok(data) if data.is_empty() => {
            app.clear_loading();
            app.screen = Screen::Status {
                message: "No visible objects / Nessun oggetto visibile".to_string(),
                back: Box::new(Screen::SchedulingMenu),
            };
        }
        Ok(data) => {
            app.clear_loading();
            app.screen = Screen::TargetTable { rows: data, scroll: 0 };
            app.status_line = "j/k: scroll  9/Esc: back".to_string();
        }
        Err(e) => {
            app.clear_loading();
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
