use multiinput::*;
use std::process::Command;

pub(super) fn listen(commands: Vec<super::config::Command>) {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let mut command_map = vec![None; devices.keyboards.len()];

    for command in commands.iter() {
        for j in 0..devices.keyboards.len() {
            if devices.keyboards[j].name.contains(&command.keyboard) && command_map[j].is_none() {
                command_map[j] = Some(command);
            }
        }
    }
    let mut last_id = 0;
    loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(id, _, State::Pressed) => {
                    if let Some(device) = devices.keyboards.get(id) {
                        let _name = device.name.clone();
                    }
                    if id + 1 != last_id {
                        last_id = id + 1;
                        if let Some(command) = command_map[id] {
                            println!("Running command: {:?} - {:?}", command.cmd, command.args);
                            Command::new(&command.cmd)
                                .args(&command.args)
                                .spawn()
                                .ok();
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
