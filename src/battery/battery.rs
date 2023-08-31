use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct PowerSupply {
    name: String,
    r#type: String,
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

pub trait PowerSupplyInfo {
    fn info(&self) -> String;
}

pub struct Battery {
    pub path: String,
}

impl PowerSupplyInfo for Battery {
    fn info(&self) -> String {
        "Battery Info".to_string()
    }
}

impl Battery {
    pub fn new() -> Self {
        Battery {
            path: "/power/v1".to_string(),
        }
    }
}