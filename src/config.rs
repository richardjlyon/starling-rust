//! Functionality for managing a config file
//!
//!
//!
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

const FILENAME: &str = "config.yaml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: DbConfig,
    pub filename: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db: DbConfig::default(),
            filename: String::from(FILENAME),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut config: Config = Default::default();
        if Path::new(&config.filename).exists() {
            config = config.load();
        }
        config
    }

    pub fn delete() {
        let _ = fs::remove_file(FILENAME);
    }

    /// load a config file from filesystem
    pub fn load(&self) -> Self {
        let f = std::fs::File::open(&self.filename).expect("opening file");
        serde_yaml::from_reader(f).expect("decoding")
    }

    /// save a config file to filesystem
    pub fn save(&self) {
        let _ = fs::remove_file(&self.filename);
        let yaml = serde_yaml::to_string(&self).unwrap();
        let mut output = fs::File::create(&self.filename).unwrap();
        write!(output, "{}", yaml).unwrap();
    }

    pub fn db_url(&self) -> String {
        format!(
            "mysql://{}:{}@db.kingswood:3306/{}",
            self.db.user, self.db.password, self.db.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Config::new();
        assert_eq!(c.filename, FILENAME);
    }

    #[test]
    fn it_saves() {
        let mut c = Config::new();
        c.filename = String::from("test.yaml");
        c.save();
        assert!(Path::new("test.yaml").exists());
        let _ = fs::remove_file("test.yaml");
    }

    #[test]
    fn it_generates_a_url() {
        let c = Config {
            db: DbConfig {
                user: String::from("admin"),
                password: String::from("password"),
                name: String::from("test_db"),
            },
            filename: String::from("test.yaml"),
        };

        assert_eq!(
            c.db_url(),
            "mysql://admin:password@db.kingswood:3306/test_db"
        );
    }
}
