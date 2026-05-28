// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

use asteroid_tui::tui;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    println!("Welcome to Asteroid_tui! / Benvenuto in Asteroid_tui!");
    println!("Version 0.1.0");
    if let Err(e) = tui::main_menu() {
        eprintln!("Error: {} / Errore: {}", e, e);
        std::process::exit(1);
    }
}
