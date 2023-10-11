use log::{error, info, trace};
use log4rs;
use serde_yaml;

fn main() {
    println!("hello");
}

#[test]
fn test_yaml() {
    let config_str = include_str!("sample_config.yml");
    let config = serde_yaml::from_str(config_str).unwrap();

    log4rs::init_raw_config(config).unwrap();

    info!("Yaml to console");
    error!("Yaml to console");
    trace!("Doesn't go to console as it is filtered out");
}

#[test]
fn test_toml() {
    let config_str = include_str!("sample_config.toml");
    let config = toml::from_str(config_str).unwrap();

    log4rs::init_raw_config(config).unwrap();

    info!("Toml to console");
    error!("Toml to console");
    trace!("Doesn't go to console as it is filtered out");
}
