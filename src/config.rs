
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_key: String,
    pub database_url_key: String,
}

impl Config {
    pub fn parse() -> Self {
        let config_path:String=std::env::var(key:"CONFIG_FILE").expect("Could not find the file");

        let config_yml=std::fs::read_to_string(&config_path).expect("yml file do not find");
    }
}
