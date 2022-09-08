use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Engine {
    pub min_year: Option<usize>,
    pub max_year: Option<usize>,
    pub max_cylinders: Option<usize>,
    pub turbo_max_cylinders: Option<usize>,
    pub octane: Option<f32>,
}

impl Engine {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        if let Some(min_year) = self.min_year {
            if car.engine_year < min_year {
                return Err(CheckError::ErrEngine(format!("engine year {}", car.engine_year)).into());
            }
        }
        if let Some(max_year) = self.max_year {
            if car.engine_year > max_year {
                return Err(CheckError::ErrEngine(format!("engine year {}", car.engine_year)).into());
            }
        }
        if let Some(max_cylinders) = self.max_cylinders {
            if car.cylinder_count > max_cylinders {
                return Err(CheckError::ErrEngine(format!("cylinder count {}", car.cylinder_count)).into());
            }
        }
        if let Some(turbo_max_cylinders) = self.turbo_max_cylinders {
            if car.has_turbo() && car.cylinder_count > turbo_max_cylinders {
                return Err(CheckError::ErrEngine(String::from("turbo_max_cylinders")).into());
            }
        }
        if let Some(octane) = self.octane {
            if car.octane > octane {
                return Err(CheckError::ErrEngine(format!("fuel octane {}", car.octane)).into());
            }
        }

        Ok(())
    }
}
