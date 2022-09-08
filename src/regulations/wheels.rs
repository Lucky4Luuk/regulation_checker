use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Wheels {
    pub must_match: Option<bool>,

    pub front_max_width: Option<usize>,
    pub front_max_profile: Option<usize>,
    pub front_max_rim: Option<usize>,

    pub rear_max_width: Option<usize>,
    pub rear_max_profile: Option<usize>,
    pub rear_max_rim: Option<usize>,
}

impl Wheels {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        if self.must_match.unwrap_or(false) {
            if !car.wheels_match() { return Err(CheckError::ErrWheels(String::from("wheels dont match")).into()); }
        }
        if let Some(front_max_width) = self.front_max_width {
            if car.wheels_front_width > front_max_width {
                return Err(CheckError::ErrWheels(format!("front tire width {}", car.wheels_front_width)).into());
            }
        }
        if let Some(front_max_profile) = self.front_max_profile {
            if car.wheels_front_profile > front_max_profile {
                return Err(CheckError::ErrWheels(format!("front tire profile {}", car.wheels_front_profile)).into());
            }
        }
        if let Some(front_max_rim) = self.front_max_rim {
            if car.wheels_front_rim > front_max_rim {
                return Err(CheckError::ErrWheels(format!("front tire rim {}", car.wheels_front_rim)).into());
            }
        }

        if let Some(rear_max_width) = self.rear_max_width {
            if car.wheels_rear_width > rear_max_width {
                return Err(CheckError::ErrWheels(format!("rear tire width {}", car.wheels_rear_width)).into());
            }
        }
        if let Some(rear_max_profile) = self.rear_max_profile {
            if car.wheels_rear_profile > rear_max_profile {
                return Err(CheckError::ErrWheels(format!("rear tire profile {}", car.wheels_rear_profile)).into());
            }
        }
        if let Some(rear_max_rim) = self.rear_max_rim {
            if car.wheels_rear_rim > rear_max_rim {
                return Err(CheckError::ErrWheels(format!("rear tire rim {}", car.wheels_rear_rim)).into());
            }
        }

        Ok(())
    }
}
