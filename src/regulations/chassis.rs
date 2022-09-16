use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Chassis {
    pub banned_materials: Option<Vec<String>>,
}

impl Chassis {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if let Some(banned_materials) = &self.banned_materials {
            for mat in banned_materials {
                if mat.trim() == car.chassis_material.trim() {
                    errs.push(format!("banned chassis material {}", car.chassis_material));
                }
            }
        }
        if errs.len() > 0 {
            return Err(CheckError::ErrChassis(errs).into());
        }

        Ok(())
    }
}
