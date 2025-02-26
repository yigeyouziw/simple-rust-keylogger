use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread::sleep, time::Duration};

fn main() {
    let device_state = DeviceState::new();
    let mut last_keys = vec![];

    println!("Keyboard logger started (Press ESC to exit)...");

    loop {
        let current_keys: Vec<Keycode> = device_state.get_keys();
        
        if current_keys != last_keys {
            for key in &current_keys {
                if !last_keys.contains(key) {
                    let key_str = match key {
                        other => format!("{:?}", other),
                    };
                    println!("KEY: {}", key_str);
                }
            }

            if current_keys.contains(&Keycode::Escape) {
                println!("ESC key detected to exit the program...");
                break;
            }

            last_keys = current_keys.clone();
        }

        sleep(Duration::from_millis(10));
    }
}
