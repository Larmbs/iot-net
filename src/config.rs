use serde::{Deserialize, Serialize};
use std::{net, path::Path, fs};
use serde_json;
use anyhow::{Context, Result};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub addr: net::IpAddr,
    pub port: u32,
}
impl Config {
    pub fn get_socket_addr(&self) -> Result<net::SocketAddr> {
        format!("{}:{}", self.addr, self.port).parse().context("Failed to read provided server information")
    }
}

/// Gets config data
pub fn load_config<T: AsRef<Path>>(path: T) -> Result<Config> {
    serde_json::from_reader(fs::File::open(path).context("Unable to read value")?).context("Unable to deserialize data")
}
