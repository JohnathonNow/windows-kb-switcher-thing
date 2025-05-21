use multiinput::*;
use std::process::Command;
use std::thread;
use tray_icon::{Icon, TrayIconBuilder, menu::IconMenuItemBuilder, menu::Menu};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

#[derive(Debug)]
enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

#[derive(Default)]
struct KbApp {}

impl KbApp {
    fn new() -> Self {
        Self::default()
    }
}

impl ApplicationHandler<UserEvent> for KbApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        // println!("User event: {:?}", event);
        match event {
            UserEvent::MenuEvent(event) => {
                if event.id() == "1" {
                    std::process::exit(0);
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let _handler = thread::spawn(move || {
        let mut manager = RawInputManager::new().unwrap();
        manager.register_devices(DeviceType::Keyboards);
        let devices = manager.get_device_list();
        let mut last_id = 0;
        'outer: loop {
            // println!("Waiting for events... {:?}", devices.keyboards);
            if let Some(event) = manager.get_event() {
                // println!("Event: {:?}", event);
                match event {
                    RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                    RawEvent::KeyboardEvent(id, _, State::Pressed) => {
                        if let Some(device) = devices.keyboards.get(id) {
                            let name = device.name.clone();
                            println!("{}", name);
                        }
                        if id != last_id {
                            last_id = id;
                            if id == 0 {
                                Command::new("cmd.exe")
                                    .args(["/c", "normal.bat"])
                                    .spawn()
                                    .ok();
                            } else {
                                Command::new("cmd.exe").args(["/c", "tv.bat"]).spawn().ok();
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    });

    let tray_menu = Menu::new();
    let item = IconMenuItemBuilder::new()
        .id("1".into())
        .text("Exit")
        .enabled(true)
        .build();
    tray_menu.append(&item).ok();
    let icon = Icon::from_rgba(vec![0, 0, 0, 0], 1, 1).unwrap();
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event)).ok();
    }));
    // handler.join().unwrap();
    let mut app = KbApp::new();
    // let handler = thread::spawn(move || {
    event_loop.run_app(&mut app).ok();
    // });
    println!("Finishing");
}
