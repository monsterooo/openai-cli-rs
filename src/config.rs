use std::fs;
use homedir::get_my_home;

use serde::{Deserialize, Serialize};
use anyhow::Result;

const FILE_NAME: &str = "openai-cli-rs.toml";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub openai_api_key: String
}

impl Config {
    pub fn load() -> Self {
        let path = get_my_home().unwrap().unwrap().join(FILE_NAME);
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(content.as_str()).unwrap();
        
        config
    }

    pub fn save(&self) -> Result<()> {
        let path = get_my_home().unwrap().unwrap().join(FILE_NAME);
        let config = toml::to_string(&self).unwrap();
        
        fs::write(path, config).unwrap();

        Ok(())
    }
}