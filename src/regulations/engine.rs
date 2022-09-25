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
    pub fuel_type: Option<Vec<String>>,

    pub max_displacement: Option<usize>,
    pub max_stroke: Option<f32>,
    pub max_bore: Option<f32>,
    pub max_intake_valves: Option<usize>,
    pub max_exhaust_valves: Option<usize>,
    pub max_total_valves: Option<usize>,
    pub max_compression: Option<f32>,
    pub max_rpm: Option<usize>,

    pub catalytic_converter_banned: Option<Vec<String>>,
    pub intake_banned: Option<Vec<String>>,
    pub headers_banned: Option<Vec<String>>,
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
        if let Some(allowed_fuels) = &self.fuel_type {
            for f in allowed_fuels {
                if f.trim() == car.fuel_type {
                    errs.push(format!("illegal fuel type {}", car.fuel_type));
                }
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
        if let Some(max_compression) = self.max_compression {
            if car.compression > max_compression {
                errs.push(format!("compression {}", car.compression));
            }
        }
        if let Some(max_rpm) = self.max_rpm {
            if car.rpm > max_rpm {
                errs.push(format!("max rpm {}", car.rpm));
            }
        }

        if let Some(catalytic_converter_banned) = &self.catalytic_converter_banned {
            for cat in catalytic_converter_banned {
                if car.catalytic_converter.to_lowercase().contains(&cat.trim().to_lowercase()) {
                    errs.push(format!("illegal cat conv {}", car.catalytic_converter))
                }
            }
        }
        if let Some(intake_banned) = &self.intake_banned {
            for intake in intake_banned {
                if car.intake_type.to_lowercase().contains(&intake.trim().to_lowercase()) {
                    errs.push(format!("illegal intake {}", car.intake_type));
                }
            }
        }
        if let Some(headers_banned) = &self.headers_banned {
            for header in headers_banned {
                if car.headers.trim() == header.trim() {
                    errs.push(format!("illegal headers {}", car.headers));
                }
            }
        }

        if errs.len() > 0 { return Err(CheckError::ErrEngine(errs).into()); }

        Ok(())
    }
}
