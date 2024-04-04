//! Functions for reading and writing bkmk state
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::{path::PathBuf, process};
extern crate home;

/// Find the location of the state file. Should be `~/.bkmk`.
fn state_file_location() -> PathBuf {
    let mut location = match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path,
        _ => {
            eprintln!("Unable to locate home directory");
            process::exit(2);
        }
    };
    location.push(".bkmk");
    location
}

/// Read the file located by `state_file_location` and return it as a HashMap.
pub fn load_state_file() -> HashMap<String, String> {
    let location = state_file_location();
    if !&location.exists() {
        return HashMap::new();
    }
    let contents = fs::read_to_string(&location).unwrap_or_else(|e| {
        eprintln!("Could not read contents of {}: {e}", location.display());
        process::exit(3)
    });
    let state: HashMap<String, String> = serde_yaml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("Could not deserialize state: {e}");
        process::exit(4)
    });
    state
}

/// Write the hashmap to the file located by `state_file_location` (serialized as yaml).
pub fn write_state_file(state: HashMap<String, String>) {
    let serialized_state = serde_yaml::to_string(&state).unwrap();
    let location = state_file_location();
    fs::write(&location, serialized_state).unwrap_or_else(|e| {
        eprintln!("Could not write state file: {e}");
        process::exit(5);
    });
    println!("Wrote state to {}", location.display());
}
