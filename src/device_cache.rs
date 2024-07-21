use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fs};
use uuid::Uuid;

pub type ID = String;

/// A JSON file holding a mapping of all device names to their assigned IDs
#[derive(Serialize, Deserialize)]
pub struct DevicesCache {
    pub name_id_map: HashMap<String, ID>,
}
impl DevicesCache {
    const CACHE_PATH: &'static str = "data/devices.json";

    /// Loads a device cache
    pub fn load() -> Result<DevicesCache> {
        let file = fs::File::open(DevicesCache::CACHE_PATH)?;
        serde_json::from_reader(file).context("File contains error")
    }

    /// Saves the device cache to dir
    pub fn save(&self) -> Result<()> {
        // Open the file with write permissions and truncation
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(DevicesCache::CACHE_PATH)
            .context("Failed to open cache file for writing")?;
        
        serde_json::to_writer(&file, self).context("Failed to write cache to file")?;
        Ok(())
    }
}

pub fn get_device_id_map() -> Result<HashMap<String, ID>> {
    Ok(DevicesCache::load()?.name_id_map)
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
    let mut cache = DevicesCache::load()?;

    // Check if the device already exists
    if cache.name_id_map.contains_key(device_name) {
        return Err(anyhow!("A device with that name already exists"));
    }

    // Add the new device to the cache
    cache.name_id_map.insert(device_name.to_string(), id.clone());

    cache.save()?;
    Ok(id)
}
