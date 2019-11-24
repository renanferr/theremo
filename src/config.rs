extern crate serde_yaml;
extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawConfig {
    keymappings: Vec<f64>,
    frequency_delta_ratio: f64
}

pub struct TheremoConfig {
    pub keymappings: HashMap<u8, f64>,
    pub frequency_delta_ratio: f64,
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
    let keys: [u8; 9] = [
        97,     // a
        115,    // s
        100,    // d
        102,    // f
        103,    // g
        104,    // h
        106,    // j
        107,    // k
        108,    // l
    ];

    for (i, frequency) in configs.keymappings.iter().enumerate() {
        notes.insert(keys[i], *frequency);
    }

    TheremoConfig {
        keymappings: notes,
        frequency_delta_ratio: configs.frequency_delta_ratio,
    }
}

// pub const KEY_NOTES: [KeyNote; 9] = [
//     KeyNote {
//         key: 97,
//         frequency: 261.63,
//     },
//     KeyNote {
//         key: 115,
//         frequency: 293.66,
//     },
//     KeyNote {
//         key: 100,
//         frequency: 329.63,
//     },
//     KeyNote {
//         key: 102,
//         frequency: 349.23,
//     },
//     KeyNote {
//         key: 103,
//         frequency: 392.0,
//     },
//     KeyNote {
//         key: 104,
//         frequency: 440.0,
//     },
//     KeyNote {
//         key: 106,
//         frequency: 493.88,
//     },
//     KeyNote {
//         key: 107,
//         frequency: 523.25,
//     },
//     KeyNote {
//         key: 108,
//         frequency: 587.33,
//     },
// ];

// #[derive(Debug, PartialEq)]
// pub struct KeyNote {
//     pub key: u8,
//     pub frequency: f64,
// }