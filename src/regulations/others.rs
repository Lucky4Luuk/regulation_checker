use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Others {
    pub banned_parts: Option<Vec<String>>,
}

impl Others {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if let Some(banned_parts) = &self.banned_parts {
            if banned_parts.len() > 0 {
                for (k, v) in &car.raw {
                    for p in banned_parts {
                        if p.clone().trim() == v {
                            errs.push(format!("banned part {}", v));
                        }
                    }
                }
            }
        }
        if errs.len() > 0 {
            return Err(CheckError::ErrOthers(errs).into());
        }

        Ok(())
    }
}
