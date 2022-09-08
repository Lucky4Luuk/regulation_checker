use anyhow::Result;

use serde::Deserialize;

use crate::specs::Car;

mod stats;
mod engine;
mod wheels;

use stats::*;
use engine::*;
use wheels::*;

#[derive(Debug, Clone)]
pub enum CheckError {
    ErrStats(Vec<String>),
    ErrEngine(String),
    ErrWheels(String),
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
    pub engine: Option<Engine>,
    pub wheels: Option<Wheels>,
}

impl Regulations {
    pub fn load() -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string("regulations.toml")?)?)
    }

    pub fn check_car(&self, car: &Car) -> Result<()> {
        if let Some(stats) = &self.stats {
            stats.check_car(car, &self.rules)?;
        }
        if let Some(engine) = &self.engine {
            engine.check_car(car)?;
        }
        if let Some(wheels) = &self.wheels {
            wheels.check_car(car)?;
        }
        Ok(())
    }
}
