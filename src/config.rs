use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub repo_name: String,
    pub user_email: String,
}

pub fn load_config() -> Result<Config, std::io::Error> {
    let config_data = fs::read_to_string(".chakconfig")?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}
