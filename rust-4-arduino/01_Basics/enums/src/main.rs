fn main() {
    
    enum DeviceState {
        ON,
        OFF,
        ERROR
    }

    let current_state: DeviceState = DeviceState::OFF;

    match current_state {
        DeviceState::ON => println!("The device is ON"),
        DeviceState::OFF => println!("The device is OFF"),
        DeviceState::ERROR => println!("The device is in ERROR state"),
        //_ => println!("Unknown device state"),
    }

}
