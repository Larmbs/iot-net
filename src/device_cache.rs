use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Seek, SeekFrom};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
};
use uuid::Uuid;

pub type ID = String;

const CACHE_PATH: &str = "./data/devices.json";

#[derive(Serialize, Deserialize)]
pub struct DevicesCache {
    pub devices_id_map: HashMap<String, ID>,
}

/// Get device to id map
pub fn get_device_id_map() -> Result<HashMap<String, ID>> {
    let file = fs::File::open(CACHE_PATH)?;
    let cache: DevicesCache = serde_json::from_reader(file)?;
    Ok(cache.devices_id_map)
}

/// Gets a devices names id
pub fn get_device_id(device_name: &String) -> Result<ID> {
    Ok(get_device_id_map()?
        .get(device_name)
        .context("Could not find a device with that name")?
        .to_string())
}

/// Adds a device to the cache and returns an ID
pub fn add_device_get_id(device_name: &String) -> Result<ID> {
    let id = Uuid::new_v4().to_string();

    // Open the file in read-write mode
    let mut file = OpenOptions::new().read(true).write(true).open(CACHE_PATH)?;

    // Read the cache from the file
    let mut cache: DevicesCache = serde_json::from_reader(&file)?;

    // Check if the device already exists
    if cache.devices_id_map.contains_key(device_name) {
        return Err(anyhow!("A device with that name already exists"));
    }

    // Add the new device to the cache
    cache
        .devices_id_map
        .insert(device_name.to_string(), id.clone());

    // Truncate the file to 0 bytes and write the updated cache
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(&file, &cache)?;

    // Return the new device ID
    Ok(id)
}
