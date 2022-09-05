use std::collections::HashMap;
use anyhow::Result;

use serde::Deserialize;

use crate::specs::Car;

#[derive(Debug, Clone)]
pub enum CheckError {
    ErrStats(Vec<String>),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl std::error::Error for CheckError {}

#[derive(Debug, Deserialize)]
pub struct Regulations {
    pub stats: Option<Stats>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub safety: Option<f32>,
    pub safety_max: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub or: Option<Vec<Vec<String>>>
}

impl Regulations {
    pub fn load() -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string("regulations.toml")?)?)
    }

    pub fn check_car(&self, car: &Car) -> Result<()> {
        if let Some(stats) = &self.stats {
            stats.check_car(car, &self.rules)?;
        }
        Ok(())
    }
}

impl Stats {
    pub fn check_car(&self, car: &Car, rules: &Option<Rules>) -> Result<()> {
        let mut result_map: HashMap<String, bool> = HashMap::new();
        result_map.insert(String::from("safety"), true);
        result_map.insert(String::from("safety_max"), true);
        if let Some(safety) = self.safety {
            if car.safety_rating < safety { result_map.insert(String::from("safety"), false); }
        }
        if let Some(safety_max) = self.safety_max {
            if car.safety_rating > safety_max { result_map.insert(String::from("safety_max"), false); }
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
                            let mut failed = true;
                            for key in rule {
                                if let Some(value) = result_map.get(key) {
                                    if *value { failed = false; }
                                }
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
