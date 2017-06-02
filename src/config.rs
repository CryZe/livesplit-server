use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;
use serde_yaml::from_reader;
use layout::LayoutSettings;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub splits: HashMap<String, PathBuf>,
    pub default_splits: String,
    pub hotkeys: bool,
    pub layout: LayoutSettings,
}

pub fn load() -> Config {
    let file = BufReader::new(File::open("config.yml").unwrap());
    from_reader(file).unwrap()
}
