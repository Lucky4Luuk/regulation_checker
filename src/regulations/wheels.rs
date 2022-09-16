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
    pub front_wheel_diameter: Option<usize>,

    pub rear_max_width: Option<usize>,
    pub rear_max_profile: Option<usize>,
    pub rear_max_rim: Option<usize>,
    pub rear_wheel_diameter: Option<usize>,

    pub banned_compounds: Option<Vec<String>>,
    pub banned_rim_materials: Option<Vec<String>>,
}

impl Wheels {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if self.must_match.unwrap_or(false) {
            if !car.wheels_match() { errs.push(String::from("wheels dont match")); }
        }

        if let Some(front_max_width) = self.front_max_width {
            if car.wheels_front_width > front_max_width {
                errs.push(format!("front tire width {}", car.wheels_front_width));
            }
        }
        if let Some(front_max_profile) = self.front_max_profile {
            if car.wheels_front_profile > front_max_profile {
                errs.push(format!("front tire profile {}", car.wheels_front_profile));
            }
        }
        if let Some(front_max_rim) = self.front_max_rim {
            if car.wheels_front_rim > front_max_rim {
                errs.push(format!("front tire rim {}", car.wheels_front_rim));
            }
        }
        if let Some(front_wheel_diameter) = self.front_wheel_diameter {
            if car.wheels_front_diameter > front_wheel_diameter {
                errs.push(format!("front tire diameter {}", car.wheels_front_diameter));
            }
        }

        if let Some(rear_max_width) = self.rear_max_width {
            if car.wheels_rear_width > rear_max_width {
                errs.push(format!("rear tire width {}", car.wheels_rear_width));
            }
        }
        if let Some(rear_max_profile) = self.rear_max_profile {
            if car.wheels_rear_profile > rear_max_profile {
                errs.push(format!("rear tire profile {}", car.wheels_rear_profile));
            }
        }
        if let Some(rear_max_rim) = self.rear_max_rim {
            if car.wheels_rear_rim > rear_max_rim {
                errs.push(format!("rear tire rim {}", car.wheels_rear_rim));
            }
        }
        if let Some(rear_wheel_diameter) = self.rear_wheel_diameter {
            if car.wheels_rear_diameter > rear_wheel_diameter {
                errs.push(format!("rear tire diameter {}", car.wheels_rear_diameter));
            }
        }

        if let Some(banned_compounds) = &self.banned_compounds {
            for compound in banned_compounds {
                if compound.clone().trim() == car.wheels_compound.trim() {
                    errs.push(format!("banned compound {}", car.wheels_compound));
                }
            }
        }
        if let Some(banned_rim_materials) = &self.banned_rim_materials {
            for mat in banned_rim_materials {
                if car.wheels_rim_material.trim() == mat.trim() {
                    errs.push(format!("banned rim material {}", car.wheels_rim_material));
                }
            }
        }

        if errs.len() > 0 {
            return Err(CheckError::ErrWheels(errs).into());
        }

        Ok(())
    }
}
