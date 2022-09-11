use anyhow::Result;
use serde::Deserialize;

use crate::specs::Car;
use super::CheckError;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ReqPart {
    Part(String),
    Choice(Vec<String>),
}

impl ReqPart {
    fn validate(&self, v: String) -> bool {
        match self {
            Self::Part(s) => s.trim() == v.trim(),
            Self::Choice(parts) => {
                for s in parts {
                    if s.trim() == v.trim() { return true; }
                }
                false
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Others {
    pub required_parts: Option<Vec<ReqPart>>,
    pub banned_parts: Option<Vec<String>>,
    pub min_quality: Option<isize>,
    pub max_quality: Option<isize>,
}

impl Others {
    pub fn check_car(&self, car: &Car) -> Result<()> {
        let mut errs = Vec::new();
        let min_quality = self.min_quality.unwrap_or(-15);
        let max_quality = self.max_quality.unwrap_or(15);
        for (k, v) in &car.raw {
            if k.trim().ends_with("_quality") {
                // worst code ive ever written
                if let Ok(v) = v.trim().parse::<f32>() {
                    let v = v as isize;
                    if !(v >= min_quality && v <= max_quality) {
                        errs.push(format!("illegal quality {}", k));
                    }
                } else {
                    errs.push(format!("incorrect int {}", k));
                }
            }
        }
        if let Some(banned_parts) = &self.banned_parts {
            if banned_parts.len() > 0 {
                for (_, v) in &car.raw {
                    for p in banned_parts {
                        if p.clone().trim() == v {
                            errs.push(format!("banned part {}", v));
                        }
                    }
                }
            }
        }
        if let Some(required_parts) = &self.required_parts {
            let mut req_parts_clone = required_parts.clone();
            for (_, v) in &car.raw {
                for i in 0..req_parts_clone.len() {
                    if req_parts_clone[i].validate(v.clone()) {
                        req_parts_clone.remove(i);
                    }
                }
            }
            if req_parts_clone.len() > 0 {
                errs.push(format!("parts missing {:?}", req_parts_clone));
            }
        }
        if errs.len() > 0 {
            return Err(CheckError::ErrOthers(errs).into());
        }

        Ok(())
    }
}
