//allow dead code, allow unused_imports, allow unused_variables, allow unused_mut
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

#[derive(Debug)]
pub struct PowerInfo {
    name: String,
    supply_type: String,
    status: String,
    present: bool,
    voltage_now: u32,
    current_now: i32,
    capacity: u8,
    capacity_level: String,
    temp: i32,
    technology: String,
    charge_full: u32,
    charge_now: u32,
    charge_full_design: u32,
    manufacturer: String,
}

impl PowerInfo {
    pub fn new() -> Self {
        // Initialize PowerSupply fields with default values here
        PowerInfo {
            name: String::new(),
            supply_type: String::new(),
            status: String::new(),
            present: false,
            voltage_now: 0,
            current_now: 0,
            capacity: 0,
            capacity_level: String::new(),
            temp: 0,
            technology: String::new(),
            charge_full: 0,
            charge_now: 0,
            charge_full_design: 0,
            manufacturer: String::new(),
        }
    }

    // Add methods to set and get individual fields as needed
}

pub struct BatteryModule {
    pub path: String,
}

impl BatteryModule {
    pub fn new() -> Self {
        BatteryModule {
            path: String::new(),
        }
    }
}

pub trait PowerSupplyInfo {
    fn info(&self) -> String;
    fn set_device(&mut self, device: &str);
    fn get_device(&self) -> &str;
}

impl PowerSupplyInfo for BatteryModule {
    fn info(&self) -> String {
        "Battery Info".to_string()
    }

    fn set_device(&mut self, device: &str) {
        self.path = device.to_owned();
    }

    fn get_device(&self) -> &str {
        &self.path
    }
}
