use super::device_cache;
use anyhow::{anyhow, Context, Result};
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::net;

#[derive(Serialize, Deserialize)]
pub struct Device {
    name: String,
    description: String,
    address: net::SocketAddr,
    sensors: Vec<Sensor>,
    config: HashMap<String, String>,
}
impl Device {
    pub fn add_entry(&mut self, sensor_name: &String, entry: Entry) -> Result<()> {
        let sensor = self
            .sensors
            .iter_mut()
            .filter(|s| &s.name == sensor_name)
            .nth(0)
            .context("Could not find sensor with that name")?;
        sensor.add_entry(entry)?;
        Ok(())
    }
}
#[derive(Serialize, Deserialize)]
enum EntryType {
    String,
    Float,
    Integer,
}

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    name: String,
    description: String,
    entry_type: EntryType,
    entries: Vec<Entry>,
}
impl Sensor {
    fn add_entry(&mut self, entry: Entry) -> Result<()> {
        if !entry.is_valid_type(&self.entry_type) {
            return Err(anyhow!("Entry type does not match Sensor's entry_type"));
        }
        self.entries.push(entry);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    value: String,
    time: String,
}
impl Entry {
    fn is_valid_type(&self, expected_type: &EntryType) -> bool {
        match expected_type {
            EntryType::String => true, // All strings are valid
            EntryType::Float => self.value.parse::<f64>().is_ok(),
            EntryType::Integer => self.value.parse::<i64>().is_ok(),
        }
    }
}

/// Saves a new database for device
pub fn save_device(device: &Device) -> Result<device_cache::ID> {
    let id = device_cache::add_device_get_id(&device.name)?;
    let mut file = fs::File::create_new(format!("./data/devices/{}.db", id))?;
    file.write(&bincode::serialize(device)?)?;
    Ok(id)
}

/// Adds an entry into a devices sensor data logs
pub fn add_entry(
    id: &device_cache::ID,
    sensor_name: &String,
    entry: Entry,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("./data/devices/{}.db", id.trim());

    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    let mut device_data: Device = bincode::deserialize_from(&file)?;

    device_data.add_entry(&sensor_name, entry)?;

    file.set_len(0)?;
    bincode::serialize_into(&file, &device_data)?;

    Ok(())
}
