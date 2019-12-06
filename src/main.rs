extern crate rumble;
// extern crate rand;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
// use rand::{Rng, thread_rng};
use rumble::api::{BDAddr, Central, Peripheral, UUID};
use rumble::bluez::manager::Manager;

pub fn main() {
    let mut addrarr = [0x24u8, 0x35u8, 0xccu8, 0x12u8, 0xd6u8, 0x34u8];
    addrarr.reverse();
    let light_addr = BDAddr {
        address: addrarr,
    };
    println!("Light addr: {}", light_addr);

    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    println!("Adapter: {}, {}", adapter.addr, adapter.is_up());
    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();
    println!("Adapter restarted");

    // connect to the adapter
    let central = adapter.connect().unwrap();
    println!("Adapter connected");

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    println!("Scanning...");
    for i in 1..15 {
        thread::sleep(Duration::from_secs(1));
        print!("{} ", i);
        io::stdout().flush().unwrap();
    }
    for peripheral in central.peripherals().into_iter() {
        println!("Found: {}", peripheral);
        if peripheral.address() == light_addr {
            println!("THIS");
        }
    }
    // find the device we're interested in
    let light = central
        .peripherals()
        .into_iter()
        .find(|p| p.address() == light_addr)
        .unwrap();
    println!("Peripheral {}", light);

    // connect to the device
    light.connect().unwrap();
    println!("Connected.");
    // discover characteristics
    light.discover_characteristics().unwrap();

    println!("Discovered characteristics");
    // find the characteristic we want
    let characteristics = light.characteristics();
    for characteristic in characteristics.iter() {
        println!("Char: {}", characteristic);
    }
    let cmd_char = characteristics.iter().find(|c| c.uuid == UUID::B16(0xFFF1)).unwrap();
    println!("Turn lights on");
    let on_cmd = vec![0x01, 0x01, 0x01, 0x01];
    light.command(&cmd_char, &on_cmd);

    #[warn(unused_must_use)]
    thread::sleep(Duration::from_secs(2));
    let off_cmd = vec![0x01, 0x01, 0x01, 0x00];
    #[warn(unused_must_use)]
    println!("Turn lights off");
    light.command(&cmd_char, &off_cmd);
    //     // dance party
    //     let mut rng = thread_rng();
    //     for _ in 0..20 {
    //        let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //        light.command(&cmd_char, &color_cmd).unwrap();
    //        thread::sleep(Duration::from_millis(200));
    //    }
}
