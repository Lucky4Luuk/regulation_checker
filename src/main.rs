mod specs;
mod regulations;

fn main() {
    println!("Loading regulations...");
    let regulations = regulations::Regulations::load().expect("Failed to load regulations!");
    // println!("{:?}", regulations);

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

    println!("Cars found: {}", dirs.len());
    let mut cars = Vec::new();
    for dir in &dirs {
        match specs::Car::from_directory(dir) {
            Ok(car) => cars.push(car),
            Err(e) => println!("{}", e),
        }
    }

    println!("Dumping all cars to car_dump.txt...");
    let mut dump = String::new();
    for car in &cars {
        dump.push_str(&format!("{:#?}", car));
        dump.push('\n');
        dump.push('\n');
    }
    std::fs::write("car_dump.txt", dump).expect("Failed to write car dump!");

    let mut max_width = 0;
    let mut results = Vec::new();
    for car in &cars {
        max_width = max_width.max(car.car_name.len());
        results.push(regulations.check_car(&car));
    }
    let mut result_text = String::from("========================================\n Results\n========================================\n");
    for (i, car) in cars.iter().enumerate() {
        let result = &results[i];
        result_text.push_str(&format!("{: <width$}", format!("{}... ", car.car_name), width = max_width + 4));
        result_text.push_str(&format!("{:?}", result));
        result_text.push('\n');
    }
    println!("{}", result_text);
    std::fs::write("result.txt", result_text).expect("Failed to write output!");
}
