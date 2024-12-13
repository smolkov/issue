use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::workspace::WORKSPACE;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {}

// impl Default for Config {
//     fn default() -> Self {
//         Config {}
//     }
// }

impl Config {
    pub fn load() -> Result<Config> {
        let config = toml::from_str(fs::read_to_string(WORKSPACE.config())?.as_str())?;
        Ok(config)
    }
}
