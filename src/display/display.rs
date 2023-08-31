use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{Error, ErrorKind, Read, Write};

pub trait DisplayInterface {
    fn set_device(&mut self, path: &str);
    fn get_device(&self) -> &str;
    fn set_brightness(&self, brightness: u8) -> Result<(), Error>;
    fn get_brightness(&self) -> Result<u8, Error>;
    fn info(&self);
}

pub struct Display {
    pub path: String,
}

impl DisplayInterface for Display {
    fn set_device(&mut self, path: &str) {
        self.path = String::from(path);
    }

    fn get_device(&self) -> &str {
        &self.path
    }

    fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        let mut file = BufWriter::new(File::create(&self.path)?);
        println!("Set Brightness: {}", brightness);
        file.write_all(brightness.to_string().as_bytes())?;
        Ok(())
    }

    fn get_brightness(&self) -> Result<u8, Error> {
        let mut file = BufReader::new(File::open(&self.path)?);
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let brightness: u8 = match buffer.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid brightness value",
                ))
            }
        };
        println!("Get Brightness: {}", brightness);
        Ok(brightness)
    }

    fn info(&self) {
        println!("Display information");
    }
}