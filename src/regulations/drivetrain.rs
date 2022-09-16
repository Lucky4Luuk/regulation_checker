use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Drivetrain {
    pub banned_types: Option<Vec<String>>,
}

impl Drivetrain {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if let Some(banned_types) = &self.banned_types {
            for bt in banned_types {
                if car.drivetrain_type.to_lowercase().contains(&bt.trim().to_lowercase()) {
                    errs.push(format!("illegal drive type {}", car.drivetrain_type));
                }
            }
        }
        if errs.len() > 0 {
            return Err(CheckError::ErrDrivetrain(errs).into());
        }

        Ok(())
    }
}
