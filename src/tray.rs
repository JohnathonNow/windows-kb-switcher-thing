use tray_icon::{Icon, TrayIconBuilder, menu::IconMenuItemBuilder, menu::Menu};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

pub(super) struct TrayEventLoop(EventLoop<UserEvent>);

#[derive(Debug)]
enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

#[derive(Default)]
pub(super) struct Tray {}

impl Tray {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn event_loop(&mut self, event_loop: TrayEventLoop) {
        let tray_menu = Menu::new();
        let item = IconMenuItemBuilder::new()
            .id("1".into())
            .text("Exit")
            .enabled(true)
            .build();
        tray_menu.append(&item).ok();
        let icon = Icon::from_rgba(include_bytes!("../resources/icon.rgba").to_vec(), 64, 64).unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("KB Switcher Thing")
            .with_icon(icon)
            .build()
            .unwrap();
        let proxy = event_loop.0.create_proxy();
        tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
            proxy.send_event(UserEvent::MenuEvent(event)).ok();
        }));
        event_loop.0.run_app(self).ok();
    }
}

impl ApplicationHandler<UserEvent> for Tray {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // We do not need to handle this event
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        // We do not need to handle this event
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::MenuEvent(event) => {
                if event.id() == "1" {
                    std::process::exit(0);
                }
            }
        }
    }
}

pub(super) fn new_event_loop() -> TrayEventLoop {
    TrayEventLoop(EventLoop::<UserEvent>::with_user_event().build().unwrap())
}
