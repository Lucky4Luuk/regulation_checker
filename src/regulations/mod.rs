use anyhow::Result;

use serde::Deserialize;

use crate::specs::Car;

mod stats;

use stats::*;

#[derive(Debug, Clone)]
pub enum CheckError {
    ErrStats(Vec<String>),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl std::error::Error for CheckError {}

#[derive(Debug, Deserialize)]
pub struct Regulations {
    pub stats: Option<Stats>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub or: Option<Vec<Vec<String>>>
}

impl Regulations {
    pub fn load() -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string("regulations.toml")?)?)
    }

    pub fn check_car(&self, car: &Car) -> Result<()> {
        if let Some(stats) = &self.stats {
            stats.check_car(car, &self.rules)?;
        }
        Ok(())
    }
}
