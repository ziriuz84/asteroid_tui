// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Night-sky color palette and styled widget builders for the TUI.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders};

/// ASCII banner lines for the main menu.
pub const BANNER: [&str; 3] = [
    "    _/_     asteroid-tui",
    "   (-.-)    minor planet planning",
    "    \\|/     observation scheduling",
];

const SPINNER_FRAMES: [&str; 4] = ["⠋", "⠙", "⠹", "⠸"];

/// Themed styles and block builders.
pub struct Theme;

impl Theme {
    pub fn accent() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn accent_bold() -> Style {
        Self::accent().add_modifier(Modifier::BOLD)
    }

    pub fn highlight() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::DarkGray)
    }

    pub fn muted() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn error() -> Style {
        Style::default().fg(Color::Red)
    }

    pub fn success() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn table_header() -> Style {
        Self::accent_bold()
    }

    pub fn table_row_alt() -> Style {
        Style::default().bg(Color::Rgb(20, 24, 36))
    }

    pub fn block(title: &str) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Self::accent())
            .title(Span::styled(format!(" {title} "), Self::accent_bold()))
    }

    pub fn title_line(text: &str) -> Line<'static> {
        Line::from(Span::styled(text.to_string(), Self::accent_bold()))
    }

    pub fn menu_item(key: &str, label: &str) -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("  [{key}] "), Self::accent_bold()),
            Span::raw(label.to_string()),
        ])
    }

    pub fn menu_title(text: &str) -> Line<'static> {
        Line::from(vec![
            Span::raw("\n"),
            Span::styled(text.to_string(), Self::accent_bold()),
            Span::raw("\n"),
        ])
    }

    pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
        let filled = current
            .saturating_mul(width)
            .checked_div(total)
            .unwrap_or(0)
            .min(width);
        let mut bar = String::with_capacity(width + 2);
        bar.push('[');
        for i in 0..width {
            bar.push(if i < filled { '=' } else { '·' });
        }
        bar.push(']');
        bar.push_str(&format!(" {current}/{total}"));
        bar
    }

    pub fn spinner_frame(tick: u8) -> &'static str {
        SPINNER_FRAMES[(tick as usize) % SPINNER_FRAMES.len()]
    }

    pub fn label_value(key: &str, value: &str) -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("{key}: "), Self::muted()),
            Span::raw(value.to_string()),
        ])
    }

    pub fn input_display(value: &str) -> Line<'static> {
        Line::from(vec![
            Span::styled("│ ", Self::muted()),
            Span::styled(
                if value.is_empty() {
                    "_".to_string()
                } else {
                    value.to_string()
                },
                Self::accent(),
            ),
            Span::styled(" │", Self::muted()),
        ])
    }
}
