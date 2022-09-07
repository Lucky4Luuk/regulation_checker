use std::collections::HashMap;
use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub or: Option<Vec<Vec<String>>>
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub drivability: Option<f32>,
    pub sportiness: Option<f32>,
    pub reliability: Option<f32>,
    pub safety: Option<f32>,
    pub practicality: Option<f32>,
    pub comfort: Option<f32>,
}

impl Stats {
    pub fn check_car(&self, car: &Car, rules: &Option<Rules>) -> Result<()> {
        let mut result_map: HashMap<String, bool> = HashMap::new();
        result_map.insert(String::from("drivability"), true);
        result_map.insert(String::from("sportiness"), true);
        result_map.insert(String::from("reliability"), true);
        result_map.insert(String::from("safety"), true);
        result_map.insert(String::from("practicality"), true);
        result_map.insert(String::from("comfort"), true);

        if let Some(drivability) = self.drivability {
            if car.drivability_rating < drivability { result_map.insert(String::from("drivability"), false); }
        }
        if let Some(sportiness) = self.sportiness {
            if car.sportiness_rating < sportiness { result_map.insert(String::from("sportiness"), false); }
        }
        if let Some(reliability) = self.reliability {
            if car.reliability_rating < reliability { result_map.insert(String::from("reliability"), false); }
        }
        if let Some(safety) = self.safety {
            if car.safety_rating < safety { result_map.insert(String::from("safety"), false); }
        }
        if let Some(practicality) = self.practicality {
            if car.practicality_rating < practicality { result_map.insert(String::from("practicality"), false); }
        }
        if let Some(comfort) = self.comfort {
            if car.comfort_rating < comfort { result_map.insert(String::from("comfort"), false); }
        }

        let result_map = match rules {
            None => {
                result_map
            },
            Some(rules) => {
                if let Some(or) = &rules.or {
                    if or.len() == 0 {
                        result_map
                    } else {
                        for rule in or {
                            let rule_name = rule.join("|");
                            let mut failed = false;
                            for key in rule {
                                failed = failed || result_map.get(key).map(|v| *v).unwrap_or(false);
                                result_map.remove(key);
                            }
                            result_map.insert(rule_name, failed);
                        }
                        result_map
                    }
                } else {
                    result_map
                }
            }
        };

        let failed: Vec<String> = result_map.iter().flat_map(|(k,v)| {
            if !v { Some(k.clone()) } else { None }
        }).collect();
        if failed.len() > 0 {
            return Err(CheckError::ErrStats(failed).into());
        }

        Ok(())
    }
}
