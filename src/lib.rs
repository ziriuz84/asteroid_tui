// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

//! Terminal tools for minor-planet observation scheduling and planning.
//!
//! The `asteroid-tui` binary provides an interactive menu for weather forecasts,
//! sun and moon times, observatory settings, and MPC observing target lists.
//! Modules in this crate implement those features and may be reused by integrations.

#![warn(missing_docs)]

/// Application and observatory configuration ([`settings::Settings`]).
pub mod settings;

/// Interactive menus for general and observatory settings.
pub mod settings_tui;

/// Scheduling menus: weather, sun/moon times, and target lists.
pub mod scheduling_tui;

/// Sunrise, sunset, and related times from observatory coordinates.
pub mod sun_moon_times;

/// Astronomical weather forecast (7timer) for the configured site.
pub mod weather;

/// MPC What's Up observing target list parsing and display.
pub mod observing_target_list;

/// Coordinate conversion and visibility helpers.
pub mod utils;

/// Main and settings menu entry points.
pub mod tui;

/// English and Italian UI strings.
pub mod i18n;
