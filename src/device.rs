use super::device_cache;
use anyhow::{anyhow, Context, Result};
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Seek, SeekFrom};

#[derive(Serialize, Deserialize)]
pub struct Device {
    #[serde(skip_serializing, default = "String::new")]
    id: device_cache::ID,
    name: String,
    description: String,
    sensors: Vec<Sensor>,
    config: HashMap<String, String>,
}
/// General operations
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
/// Saving and loading operations for Device
impl Device {
    const DEVICES_FOLDER: &'static str = "./data/devices/";
    /// Loads a device from the devices folder given its ID
    pub fn load(id: &device_cache::ID) -> Result<Device> {
        let file = fs::File::open(format!("{}{}.db", Device::DEVICES_FOLDER, id))?;
        let mut device: Device = bincode::deserialize_from(file).context("File contains error")?;
        device.id = id.clone();
        Ok(device)
    }
    /// Loads a device from the devices folder given its name
    pub fn load_from_name(name: &String) -> Result<Device> {
        let id = device_cache::add_device_get_id(&name)?;
        Device::load(&id)
    }
    /// Saves a device to its respective location in the devices folder
    pub fn save(&self) -> Result<()> {
        assert!(
            self.id != String::new(),
            "You must use the save_as_new() method. The Device object you provided does not have its id felid filled."
        );
        let mut file = fs::File::open(format!("{}{}.db", Device::DEVICES_FOLDER, self.id))?;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        Ok(bincode::serialize_into(file, &self)?)
    }
    /// Saves a device as new meaning it will give it a new ID and add it to the cache
    pub fn save_as_new(&mut self) -> Result<device_cache::ID> {
        let id = device_cache::add_device_get_id(&self.name)?;
        self.id = id.clone();
        fs::File::create_new(format!("{}{}.db", Device::DEVICES_FOLDER, self.id))?;
        self.save()?;
        Ok(id)
    }
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
        if !entry.is_valid(&self.entry_type) {
            return Err(anyhow!("Entry type does not match Sensor's entry_type"));
        }
        self.entries.push(entry);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub enum EntryType {
    String,
    Float,
    Integer,
}
impl EntryType {
    fn validate(&self, value: &String) -> bool {
        match self {
            EntryType::String => true,
            EntryType::Float => value.parse::<f64>().is_ok(),
            EntryType::Integer => value.parse::<i64>().is_ok(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    value: String,
    time: String,
}
impl Entry {
    fn is_valid(&self, expected: &EntryType) -> bool {
        expected.validate(&self.value)
    }
}
