use crate::error::{err, raw_err, Error};
use prelude::logger::make_fatal;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};
use tl::{eval, Source};
use userd::User;

#[derive(Debug, Deserialize)]
pub struct PartialConfig {
    pub users: HashMap<String, User>,
}

pub fn read() -> Result<PartialConfig, Error> {
    let source = match Source::from_path(PathBuf::from("/config/system.tl")) {
        Ok(source) => source,
        Err(err) => return err!(NoConfig(err)),
    };

    match eval::<PartialConfig>(source).map_err(|err| raw_err!(InvalidConfig(err)))? {
        Some(config) => Ok(config),
        None => err!(InvalidConfig(Box::new(make_fatal!("Empty config")))),
    }
}
