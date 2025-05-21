use multiinput::*;
use std::process::Command;
use std::thread;

pub(super) fn listen() {
    let _handler = thread::spawn(move || {
        let mut manager = RawInputManager::new().unwrap();
        manager.register_devices(DeviceType::Keyboards);
        let devices = manager.get_device_list();
        let mut last_id = devices.keyboards.len();
        loop {
            if let Some(event) = manager.get_event() {
                match event {
                    RawEvent::KeyboardEvent(id, _, State::Pressed) => {
                        if let Some(device) = devices.keyboards.get(id) {
                            let _name = device.name.clone();
                        }
                        if id != last_id {
                            last_id = id;
                            if id == 0 {
                                Command::new("cmd.exe")
                                    .args(["/c", "normal.bat"])
                                    .spawn()
                                    .ok();
                            } else {
                                Command::new("cmd.exe")
                                    .args(["/c", "tv.bat"])
                                    .spawn()
                                    .ok();
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    });
}
