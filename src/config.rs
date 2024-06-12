use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub users_path: PathBuf,
}

pub fn read_config() -> Config {
    let config_str = include_str!("../../config.toml");
    toml::from_str::<Config>(config_str).expect("Failed to parse userspace config.")
}
