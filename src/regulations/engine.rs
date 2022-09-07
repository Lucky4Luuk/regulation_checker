use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Engine {
    pub max_cylinders: Option<usize>,
    pub turbo_max_cylinders: Option<usize>,
}

impl Engine {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        if let Some(max_cylinders) = self.max_cylinders {
            if car.cylinder_count > max_cylinders {
                return Err(CheckError::ErrEngine(String::from("cylinder_count")).into());
            }
        }
        if let Some(turbo_max_cylinders) = self.turbo_max_cylinders {
            if car.has_turbo() && car.cylinder_count > turbo_max_cylinders {
                return Err(CheckError::ErrEngine(String::from("turbo_max_cylinders")).into());
            }
        }

        Ok(())
    }
}
