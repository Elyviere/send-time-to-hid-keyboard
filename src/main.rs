extern crate hidapi;

use chrono::prelude::*;
use std::time::Duration;
use std::thread;
use hidapi::HidApi;

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

fn main() {
    loop {
        let mut current_time = get_current_time();
        match run_keyboard_connection(&mut current_time) {
            Ok(_) => break,
            Err(err) => {
                eprintln!("Error: {}. Retrying next minute...", err);
                let time_to_next_minute = Duration::from_secs(60 - current_time.second as u64);
                thread::sleep(time_to_next_minute);
            }
        }
    }
}

fn run_keyboard_connection(current_time: &mut Time) -> Result<(), &'static str> {
    let api = hidapi::HidApi::new().expect("Failed to initialise HidApi");

    match connect_hid_device(&api) {
        Some(device) => {
            loop {
                write_time_to_device(&device, &current_time)?;

                let time_to_next_minute = Duration::from_secs(60 - current_time.second as u64);
                thread::sleep(time_to_next_minute);

                *current_time = get_current_time();
            }
        }
        None => Err("Device not found")
    }
}

// FalbaTech Sofle keyboard
const USAGE_PAGE: u16 = 0xFF60;
const USAGE: u16 = 0x61;
const VENDOR_ID: u16 = 0xFC32;
const PRODUCT_ID: u16 = 0x0287;

fn connect_hid_device(api: &HidApi) -> Option<hidapi::HidDevice> {
    api.device_list()
        .find(|device| {
            device.usage_page() == USAGE_PAGE 
                && device.usage() == USAGE 
                && device.vendor_id() == VENDOR_ID 
                && device.product_id() == PRODUCT_ID
        }).map(|device_info| api.open_path(device_info.path()).expect("Failed to open connection"))
}

fn write_time_to_device(device: &hidapi::HidDevice, current_time: &Time) -> Result<(), &'static str> {
    let hour_ten = (current_time.hour/10) as u8 + b'0';
    let hour_one = (current_time.hour%10) as u8 + b'0';
    let minute_ten = (current_time.minute/10) as u8 + b'0';
    let minute_one = (current_time.minute%10) as u8 + b'0';

    let buf = [0u8, hour_ten, hour_one, b':', minute_ten, minute_one];
    if let Ok(res) = device.write(&buf) {
        println!("Wrote: {:?} byte(s), at time {:02}:{:02}:{:02}", 
                 res, current_time.hour, current_time.minute, current_time.second);
        Ok(())
    } else {
        Err("Failed to write to the device")
    }
}

fn get_current_time() -> Time {
    let time = Local::now().time();
    let hour = time.hour() as u8;
    let minute = time.minute() as u8;
    let second = time.second() as u8;

    Time { hour, minute, second }
}
