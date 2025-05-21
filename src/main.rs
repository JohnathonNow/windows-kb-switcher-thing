#![windows_subsystem = "windows"]
mod keylistener;
mod tray;

fn main() {
    let event_loop = tray::new_event_loop();
    let mut tray = tray::Tray::new();
    keylistener::listen();
    tray.event_loop(event_loop);
}
