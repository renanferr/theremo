extern crate serde_yaml;
extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use std::io::Read;
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawConfig {
    notes: Vec<String>,
    keys: Vec<char>,
    glide: GlideOptions,
    notes_file: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GlideOptions {
    pub enabled: bool,
    pub ratio: f64,
}

pub struct TheremoConfig {
    pub keymappings: HashMap<u8, String>,
    pub notes: HashMap<String, f64>,
    pub glide_ratio: f64,
}

const CONFIG_FILE_PATH: &str = "./config.yaml";

fn read_notes_file(file_path: String) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut notes: HashMap<String, f64> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let note_name = record.get(0);
        let octave = record.get(1);
        let frequency = record.get(2);
        if let (Some(note_name), Some(octave), Some(frequency)) = (note_name, octave, frequency) {
            // TO DO: Add support for flat notes in CSV with a "/"
            let key = format!("{}{}", note_name, octave);
            notes.insert(key, frequency.parse().unwrap());
        }
    }

    Ok(notes)
}

fn read_config_file() -> String {
    let mut file = match std::fs::File::open(CONFIG_FILE_PATH) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            panic!(err)
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    content
}

pub fn init() -> TheremoConfig {
    let config_content = read_config_file();

    let configs: RawConfig = serde_yaml::from_str(&config_content).unwrap();

    let notes = match read_notes_file(configs.notes_file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            panic!()
        }
    };

    let mut keymappings: HashMap<u8, String> = HashMap::new();

    let keys: Vec<u8> = configs.keys.into_iter().map(|x| x as u8).collect();
    let config_notes: Vec<String> = configs.notes;

    if keys.len() != config_notes.len() {
        panic!("Mismatching keymappings lengths!");
    }

    for i in 0..keys.len() {
        keymappings.insert(keys[i], config_notes[i].to_owned());
    }

    let glide_ratio = match configs.glide.enabled {
        true => configs.glide.ratio,
        false => 1.0,
    };

    TheremoConfig {
        keymappings,
        notes,
        glide_ratio,
    }
}