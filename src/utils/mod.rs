pub mod color_utils;

use std::{fs::File, io::prelude::*};
use toml::Value;

pub fn read_config(file: &str) -> Value {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.parse::<Value>().unwrap();
}
