//! Functionality for managing a config file
//!
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: Vec<HashMap<String, String>>,
    pub db: DbConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub name: String,
}

impl Config {
    pub fn new() -> Self {
        let f = std::fs::File::open("config.yaml").expect("opening file");
        let d: Config = serde_yaml::from_reader(f).expect("decoding");

        d
    }

    pub fn db_url(&self) -> String {
        format!(
            "mysql://{}:{}@db.kingswood:3306/{}",
            self.db.user, self.db.password, self.db.name
        )
    }
}
