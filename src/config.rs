use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    ip: String,
    port: u16,
}

impl Config {
    pub fn new(file: PathBuf) -> io::Result<Self> {
        let config_file = File::open(file)?;
        let config: Config = serde_yaml::from_reader(config_file).unwrap();
        Ok(config)
    }

    pub fn ip(&self) -> String {
        self.ip.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}