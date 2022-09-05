mod specs;

fn main() {
    println!("Scanning ./cars...");
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir("cars").expect("Cannot read ./cars! Does the folder exist?") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.display().to_string());
            }
        }
    }

    println!("Converting all csv files to utf-8...");
    for dir in &dirs {
        let name = dir.replace("cars/", "").replace(r"cars\", "");
        let p = format!("{}/{}.csv", dir, name);
        let p2 = format!("{}/{}_utf8.csv", dir, name);
        let data = std::fs::read(&p).expect("Failed to read csv file!");
        let (_prefix, utf16, _suffix) = unsafe { data.align_to::<u16>() };
        let utf8 = String::from_utf16(&utf16[..]).expect("Failed to turn utf16 data into utf8! Please message me if this happens");
        std::fs::write(p2, utf8).expect("Failed to write csv!");
    }

    println!("Cars found:\n{}", dirs.join("\n"));
    let mut cars = Vec::new();
    for dir in &dirs {
        match specs::Car::from_directory(dir) {
            Ok(car) => cars.push(car),
            Err(e) => println!("{}", e),
        }
    }

    for car in &cars {
        println!("car: {}", car.car_name);
        println!("model year: {}", car.model_year);
        // println!("front brake type: {}", car.front_brake_type);
    }
}
