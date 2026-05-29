// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Terminal tools for minor-planet observation scheduling and planning.
//!
//! The `asteroid-tui` binary provides a full-screen Ratatui interface for weather forecasts,
//! sun and moon times, observatory settings, and MPC observing target lists.
//! Modules in this crate implement those features and may be reused by integrations.
//!
//! User guide: [README.md](https://github.com/ziriuz84/asteroid_tui/blob/main/README.md).
//! Developer overview: [docs/architecture.md](https://github.com/ziriuz84/asteroid_tui/blob/main/docs/architecture.md).

#![warn(missing_docs)]

/// Application and observatory configuration ([`settings::Settings`]).
pub mod settings;

/// Full-screen Ratatui application.
pub mod app;

/// Sunrise, sunset, and related times from observatory coordinates.
pub mod sun_moon_times;

/// Astronomical weather forecast (7timer) for the configured site.
pub mod weather;

/// MPC What's Up observing target list parsing and display.
pub mod observing_target_list;

/// Coordinate conversion and visibility helpers.
pub mod utils;

/// English and Italian UI strings.
pub mod i18n;
