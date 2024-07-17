use serde::{Serialize, Deserialize};
use std::io::Write;
use std::net;
use std::collections::HashMap;
use bincode;
use anyhow::{Context, Result, anyhow};
use std::fs;
use uuid;

type ID = String;

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
        let sensor = self.sensors.iter_mut().filter(|s| &s.name == sensor_name).nth(0).context("Could not find sensor with that name")?;
        sensor.add_entry(entry)?;
        Ok(())
    }
}
#[derive(Serialize, Deserialize)]
enum EntryType {
    String,
    Float, 
    Integer
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
pub fn save_device(device: &Device) -> Result<ID> {
    let id = uuid::Uuid::new_v4().to_string();
    let mut file = fs::File::create_new(format!("./data/devices/{}.db", id))?;
    file.write(&bincode::serialize(device)?)?;
    Ok(id)
}   

pub fn add_entry(id: &String, sensor_name: &String, entry: Entry) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the file path
    let file_path = format!("./data/devices/{}.db", id.trim());

    // Open the file with read-write permissions
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    // Deserialize device data from the file
    let mut device_data: Device = bincode::deserialize_from(&file)?;

    // Add entry to the device data
    device_data.add_entry(&sensor_name, entry)?;

    // Reset the file cursor and serialize updated device data back into the file
    file.set_len(0)?;
    bincode::serialize_into(&file, &device_data)?;

    Ok(())
}