// Copyright (C) 2024-2026 Sirio Negri
// SPDX-License-Identifier: GPL-3.0-or-later

use asteroid_tui::tui;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    println!("Welcome to asteroid-tui! / Benvenuto in asteroid-tui!");
    println!("Version {}", env!("CARGO_PKG_VERSION"));
    if let Err(e) = tui::main_menu() {
        eprintln!("Error: {} / Errore: {}", e, e);
        std::process::exit(1);
    }
}
