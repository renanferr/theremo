extern crate serde_yaml;
extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawConfig {
    frequencies: Vec<f64>,
    keys: Vec<char>,
    glide_ratio: f64
}

pub struct TheremoConfig {
    pub keymappings: HashMap<u8, f64>,
    pub glide_ratio: f64,
}

const CONFIG_FILE_PATH: &str = "./theremo.yaml";

pub fn init() -> TheremoConfig {
    let mut file = match std::fs::File::open(CONFIG_FILE_PATH) {
        Ok(f) => f,
        Err(err) => {
            println!("{}", err);
            panic!(err)
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let configs: RawConfig = serde_yaml::from_str(&content).unwrap();

    let mut notes: HashMap<u8, f64> = HashMap::new();
    let keys: Vec<u8> = configs.keys.into_iter().map(|x| x as u8).collect();
    let frequencies: Vec<f64> = configs.frequencies;

    if keys.len() != frequencies.len() {
        panic!("Mismatching keymappings lengths!");
    }

    for i in 0..keys.len() {
        notes.insert(keys[i], frequencies[i]);
    }    

    TheremoConfig {
        keymappings: notes,
        glide_ratio: configs.glide_ratio,
    }
}