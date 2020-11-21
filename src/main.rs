#![allow(unused_variables, unused_imports, dead_code)]

use std::io::BufReader;
use std::fs;
use std::env;
use rodio::{OutputStream, Sink, Decoder};
use cpal::{default_host, Host, Device};
use cpal::traits::{HostTrait, DeviceTrait};

fn sysdefault() -> Option<Device> {
    let sysdefault_name = String::from("sysdefault");
    match default_host().devices() {
        Ok(mut devs) => devs.find(|dev| match dev.name() {
            Ok(sysdefault_name) => true,
            _ => false
        }),
        Err(_) => None
    }
}

fn try_all_devices(filename: &String) {
    match default_host().devices() {
        Ok(devs) => for dev in devs {
            match OutputStream::try_from_device(&dev) {
                Ok((_stream, handle)) => {
                    let file = fs::File::open(filename).unwrap();
                    match handle.play_once(BufReader::new(file)) {
                        Ok(_) => println!("SUCCESS on {:?}!", dev.name()),
                        Err(_) => println!("Failed to play sound: {:?}", dev.name()),
                    }
                },
                Err(_) => println!("Failed to open device: {:?}", dev.name()),
            }
        },
        Err(_) => println!("ERROR: No devices found!"),
    }
}

fn main() {
    match env::args().nth(1) {
        Some(filename) => {
            /*
            let (_stream, handle) = OutputStream::try_default().unwrap();
            // let sink = Sink::try_new(&handle).unwrap();

            let file = fs::File::open(filename).unwrap();
            // sink.append(Decoder::new(BufReader::new(file)).unwrap());

            // sink.sleep_until_end();
            handle.play_once(BufReader::new(file)).unwrap();
            */
            match sysdefault() {
                Some(dev) => {
                    let (_stream, handle) = OutputStream::try_from_device(&dev).unwrap();

                    let file = fs::File::open(filename).unwrap();

                    handle.play_once(BufReader::new(file)).unwrap();
                },
                None => println!("ERROR: device 'sysdefault' not available")
            }
            /*
            try_all_devices(&filename);
        */
        },
        None => println!("ERROR: please provide a file name")
    }
}
