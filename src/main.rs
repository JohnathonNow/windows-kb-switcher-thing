#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

mod keylistener;
mod tray;
mod config;

fn main() {
    let config = config::get_config();
    let event_loop = tray::new_event_loop();
    let mut tray: tray::Tray = tray::Tray::new();
    thread::spawn(|| keylistener::listen(config.to_commands()));
    tray.event_loop(event_loop);
}
