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
    println!("Cars found:\n{}", dirs.join("\n"));
    let mut cars = Vec::new();
    for dir in &dirs {
        match specs::Car::from_directory(dir) {
            Ok(car) => cars.push(car),
            Err(e) => println!("{}", e),
        }
    }
}
