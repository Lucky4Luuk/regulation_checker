use anyhow::Result;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Car {
    #[serde(rename(deserialize = "Exporter Version"))]
    exporter_version: String,
    #[serde(rename(deserialize = "Car Name"))]
    car_name: String,
}

impl Car {
    pub fn from_directory(path: &str) -> Result<Self> {
        let name = path.replace("cars/", "").replace(r"cars\", "");
        let p = format!("{}/{}.csv", path, name);
        let mut rdr = csv::Reader::from_path(p)?;
        // Ok(rdr.deserialize().next().expect("How did we get here?")?)
        let mut tmp: Vec<Self> = Vec::new();
        for record in rdr.deserialize() {
            if let Ok(r) = record {
                tmp.push(r);
            }
        }
        Ok(tmp[0].clone())
    }
}
