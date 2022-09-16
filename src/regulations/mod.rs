use anyhow::Result;

use serde::Deserialize;

use crate::specs::Car;

mod stats;
mod engine;
mod wheels;
mod chassis;
mod drivetrain;
mod others;

use stats::*;
use engine::*;
use wheels::*;
use chassis::*;
use drivetrain::*;
use others::*;

#[derive(Debug)]
pub enum CheckError {
    Regulations(Vec<anyhow::Error>),
    ErrStats(Vec<String>),
    ErrEngine(Vec<String>),
    ErrWheels(Vec<String>),
    ErrChassis(Vec<String>),
    ErrDrivetrain(Vec<String>),
    ErrOthers(Vec<String>),
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
    pub chassis: Option<Chassis>,
    pub drivetrain: Option<Drivetrain>,
    pub other: Option<Others>,
}

impl Regulations {
    pub fn load() -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string("regulations.toml")?)?)
    }

    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if let Some(stats) = &self.stats {
            if let Err(e) = stats.check_car(car, &self.rules) {
                errs.push(e);
            }
        }
        if let Some(engine) = &self.engine {
            if let Err(e) = engine.check_car(car) {
                errs.push(e);
            }
        }
        if let Some(wheels) = &self.wheels {
            if let Err(e) = wheels.check_car(car) {
                errs.push(e);
            }
        }
        if let Some(chassis) = &self.chassis {
            if let Err(e) = chassis.check_car(car) {
                errs.push(e);
            }
        }
        if let Some(drivetrain) = &self.drivetrain {
            if let Err(e) = drivetrain.check_car(car) {
                errs.push(e);
            }
        }
        if let Some(others) = &self.other {
            if let Err(e) = others.check_car(car) {
                errs.push(e);
            }
        }
        if errs.len() > 0 {
            return Err(CheckError::Regulations(errs).into());
        }
        Ok(())
    }
}
