use ipc_userd::User;
use logger::{make_fatal, Log};
use serde::Deserialize;
use std::{collections::HashMap, fs};
use tl::eval;

#[derive(Debug, Deserialize)]
pub struct PartialConfig {
    pub users: HashMap<String, User>,
}

pub fn read() -> Result<PartialConfig, Box<Log>> {
    match eval::<PartialConfig>(
        fs::read_to_string("/system/config.tl")
            .map_err(|_| Box::new(make_fatal!("Failed to read config file", hint: "Check if /system/config.tl exists")))?,
    )? {
        Some(config) => Ok(config),
        None => Err(Box::new(
            make_fatal!("Failed to evaluate config file", hint: "Check if /system/config.tl is valid"),
        )),
    }
}
