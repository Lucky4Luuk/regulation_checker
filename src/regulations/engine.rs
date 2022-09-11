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

    pub max_displacement: Option<usize>,
    pub max_stroke: Option<f32>,
    pub max_bore: Option<f32>,
    pub max_intake_valves: Option<usize>,
    pub max_exhaust_valves: Option<usize>,
    pub max_total_valves: Option<usize>,
}

impl Engine {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        if let Some(min_year) = self.min_year {
            if car.engine_year < min_year {
                errs.push(format!("engine year {}", car.engine_year));
            }
        }
        if let Some(max_year) = self.max_year {
            if car.engine_year > max_year {
                errs.push(format!("engine year {}", car.engine_year));
            }
        }
        if let Some(max_cylinders) = self.max_cylinders {
            if car.cylinder_count > max_cylinders {
                errs.push(format!("cylinder count {}", car.cylinder_count));
            }
        }
        if let Some(turbo_max_cylinders) = self.turbo_max_cylinders {
            if car.has_turbo() && car.cylinder_count > turbo_max_cylinders {
                errs.push(String::from("turbo_max_cylinders"));
            }
        }
        if let Some(octane) = self.octane {
            if car.octane > octane {
                errs.push(format!("fuel octane {}", car.octane));
            }
        }

        if let Some(max_displacement) = self.max_displacement {
            if car.displacement > max_displacement {
                errs.push(format!("displacement {}", car.displacement));
            }
        }
        if let Some(max_stroke) = self.max_stroke {
            if car.stroke > max_stroke {
                errs.push(format!("stroke {}", car.stroke));
            }
        }
        if let Some(max_bore) = self.max_bore {
            if car.bore > max_bore {
                errs.push(format!("bore {}", car.bore));
            }
        }
        if let Some(max_intake_valves) = self.max_intake_valves {
            if car.intake_valves > max_intake_valves {
                errs.push(format!("intake valves {}", car.intake_valves));
            }
        }
        if let Some(max_exhaust_valves) = self.max_exhaust_valves {
            if car.exhaust_valves > max_exhaust_valves {
                errs.push(format!("exhaust valves {}", car.exhaust_valves));
            }
        }
        if let Some(max_total_valves) = self.max_total_valves {
            let total_valves = car.intake_valves + car.exhaust_valves;
            if total_valves > max_total_valves {
                errs.push(format!("total valves {}", total_valves));
            }
        }

        if errs.len() > 0 { return Err(CheckError::ErrEngine(errs).into()); }

        Ok(())
    }
}
