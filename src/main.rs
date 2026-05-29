// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

use asteroid_tui::app;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    if let Err(e) = app::run() {
        eprintln!("Error: {} / Errore: {}", e, e);
        std::process::exit(1);
    }
}
