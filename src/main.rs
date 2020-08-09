mod admin;
mod config;
mod constants;
mod rse_server;
use std::env;

pub fn main() {
    let mut url: &str = "/";
    let config_exist: bool = std::path::Path::new(constants::CONFIG_FILE).exists();
    for argument in env::args() {
        if argument == "--install" {
            url = "/rs-admin/install";
        }
    }
    if !config_exist {
        url = "/rs-admin/admin"
    }
    config::load_config();
    rse_server::start();
}
