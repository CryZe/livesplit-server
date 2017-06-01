use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde_yaml::from_reader;
use component::ComponentSettings;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub splits: PathBuf,
    pub hotkeys: bool,
    pub layout: Layout,
}

#[derive(Deserialize)]
pub struct Layout(pub Vec<ComponentSettings>);

pub fn load() -> Config {
    let file = BufReader::new(File::open("config.yml").unwrap());
    from_reader(file).unwrap()
}
