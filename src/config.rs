use std::{collections::HashMap, fs};

use config::Config;
use serde::{Deserialize, Serialize};

use crate::version_checker::BlenderVersion;

const SELF_NAME: &str = "blender_file_version_switcher";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub executable_map: HashMap<String, String>,
    pub default: Option<String>,
}
fn can_open(exec_version: &str, file_version: &str) -> bool {
    let exec_version = BlenderVersion::to_raw_version(exec_version);
    let file_version = BlenderVersion::to_raw_version(file_version);
    exec_version >= file_version
}
#[test]
fn test_can_open() {
    assert!(can_open("3.0.1", "3.0"));
    assert!(can_open("3.0", "3.0"));
    assert!(can_open("3.0", "3.0.1"));
    assert!(!can_open("2.93", "3.0"));
    assert!(!can_open("1.80", "2.93"));
    assert!(can_open("2.93", "1.80"));
    assert!(can_open("4.0", "3.0.1"));
    assert!(!can_open("3.0", "4.1.1"));
    assert!(!can_open("4.0", "4.1.1"));
}
impl Settings {
    pub fn get_executable(&self, file_version: &str) -> Option<String> {
        if let Some(exact) = self.executable_map.get(file_version) {
            return Some(exact.to_owned());
        }
        if let Some(default) = &self.default {
            if can_open(&default, file_version) {
                return self.get_executable(&default)
            }
            eprintln!("file version {} > default version {}", &file_version, &default);
        }
        None
    }
}

pub fn load() -> Result<Settings, config::ConfigError> {
    let config_dir = dirs::config_dir().unwrap()
        .join(SELF_NAME);
    let config_file = config_dir.join(CONFIG_FILE_NAME);
    if config_file.exists() {
        let settings = Config::builder()
            .add_source(config::File::from(config_file).format(config::FileFormat::Json5))
            .add_source(config::Environment::with_prefix("BSWITCH"))
            .build()
            .unwrap();

        settings.try_deserialize()
    } else {
        Ok(Settings {
            default: None,
            executable_map: HashMap::new(),
        })
    }
}

pub fn save(settings: &Settings) -> Result<(), std::io::Error> {
    let config_dir = dirs::config_dir().unwrap().join(SELF_NAME);
    fs::create_dir_all(config_dir.clone()).unwrap();
    let config_file = config_dir.join(CONFIG_FILE_NAME);

    std::fs::write(config_file, json5::to_string(settings).unwrap())
}
