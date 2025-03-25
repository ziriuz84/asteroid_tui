use asteroid_tui::tui;
use human_panic::setup_panic;
// use std::io;

fn main() {
    setup_panic!();
    println!("Welcome to Asteroid_tui!");
    let _ = tui::main_menu();
}
