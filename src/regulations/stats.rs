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
    pub model_min_year: Option<usize>,
    pub model_max_year: Option<usize>,

    pub drivability: Option<f32>,
    pub sportiness: Option<f32>,
    pub reliability: Option<f32>,
    pub safety: Option<f32>,
    pub practicality: Option<f32>,
    pub comfort: Option<f32>,
    pub prestige: Option<f32>,
    pub offroad: Option<f32>,
    pub cost: Option<f32>,
    pub service_cost: Option<f32>,
    pub fuel_economy: Option<f32>,
}

impl Stats {
    pub fn check_car(&self, car: &Car, rules: &Option<Rules>) -> Result<()> {
        let mut result_map: HashMap<String, bool> = HashMap::new();
        result_map.insert(String::from("model_min_year"), true);
        result_map.insert(String::from("model_max_year"), true);

        result_map.insert(String::from("drivability"), true);
        result_map.insert(String::from("sportiness"), true);
        result_map.insert(String::from("reliability"), true);
        result_map.insert(String::from("safety"), true);
        result_map.insert(String::from("practicality"), true);
        result_map.insert(String::from("comfort"), true);
        result_map.insert(String::from("prestige"), true);
        result_map.insert(String::from("offroad"), true);
        result_map.insert(String::from("cost"), true);
        result_map.insert(String::from("serive_cost"), true);
        result_map.insert(String::from("fuel_economy"), true);

        if let Some(model_min_year) = self.model_min_year {
            if car.model_year < model_min_year { result_map.insert(String::from("model_min_year"), false); }
        }
        if let Some(model_max_year) = self.model_max_year {
            if car.model_year > model_max_year { result_map.insert(String::from("model_max_year"), false); }
        }

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
        if let Some(prestige) = self.prestige {
            if car.prestige_rating < prestige { result_map.insert(String::from("prestige"), false); }
        }
        if let Some(offroad) = self.offroad {
            if car.offroad_rating < offroad { result_map.insert(String::from("offroad"), false); }
        }
        if let Some(cost) = self.cost {
            if car.cost > cost { result_map.insert(String::from("cost"), false); }
        }
        if let Some(service_cost) = self.service_cost {
            if car.service_cost > service_cost { result_map.insert(String::from("service_cost"), false); }
        }
        if let Some(fuel_economy) = self.fuel_economy {
            if car.fuel_economy > fuel_economy { result_map.insert(String::from("fuel_economy"), false); }
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
