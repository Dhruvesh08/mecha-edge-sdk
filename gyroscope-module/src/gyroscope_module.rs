use std::default::Default;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

#[derive(Debug)]
pub struct GyroscopeModule {
    x_axis_path: String,
    y_axis_path: String,
    z_axis_path: String,
}

impl Default for GyroscopeModule {
    fn default() -> Self {
        GyroscopeModule {
            x_axis_path: String::from("/sys/bus/iio/devices/iio:device1/in_anglvel_x_raw"),
            y_axis_path: String::from("/sys/bus/iio/devices/iio:device1/in_anglvel_y_raw"),
            z_axis_path: String::from("/sys/bus/iio/devices/iio:device1/in_anglvel_z_raw"),
        }
    }
}

impl GyroscopeModule {
    pub fn new(x_path: &str, y_path: &str, z_path: &str) -> Self {
        GyroscopeModule {
            x_axis_path: String::from(x_path),
            y_axis_path: String::from(y_path),
            z_axis_path: String::from(z_path),
        }
    }

    pub fn read_axis(&self) -> Result<(f64, f64, f64), io::Error> {
        let x_value = self.read_value_from_file(&self.x_axis_path)?;
        let y_value = self.read_value_from_file(&self.y_axis_path)?;
        let z_value = self.read_value_from_file(&self.z_axis_path)?;
        Ok((x_value, y_value, z_value))
    }

    pub fn set_default_axis(&self, x: f64, y: f64, z: f64) -> Result<(), io::Error> {
        self.write_value_to_file(&self.x_axis_path, x)?;
        self.write_value_to_file(&self.y_axis_path, y)?;
        self.write_value_to_file(&self.z_axis_path, z)?;
        Ok(())
    }

    fn read_value_from_file(&self, path: &str) -> Result<f64, io::Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let buffer_value = buffer.lines().next().unwrap()?;
        let value = buffer_value.trim().parse().unwrap();
        Ok(value)
    }

    fn write_value_to_file(&self, path: &str, value: f64) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        write!(file, "{}", value)?;
        Ok(())
    }
}
