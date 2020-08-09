use serde_json::Value;
use std::{fs, io};
use crate::constants;

pub fn load_config() -> Result<Value, io::Error> {
    println!("{}", constants::CONFIG_FILE);
    let config_str = fs::read_to_string(constants::CONFIG_FILE)?;
    println!("{:?}hello", config_str);
    let v: Value = serde_json::from_str(config_str.as_ref()).unwrap();
    println!("{:?}", v);
    Ok(v)
}

pub fn install() -> Result<Value, io::Error> {
    println!("{}", constants::CONFIG_FILE);
    let confic_str = fs::read_to_string(constants::CONFIG_FILE)?;
    println!("{:?}hello", confic_str);
    let v: Value = serde_json::from_str(confic_str.as_ref()).unwrap();
    println!("{:?}", v);
    Ok(v)
}

struct Config {
    db:DB
}

struct DB {
    name: String,
    host: String,
    user_name: String,
    password: String,
    auth_db: String,
    port: u16
}
