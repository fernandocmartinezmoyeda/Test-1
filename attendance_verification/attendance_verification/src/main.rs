use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

struct Car {
    brand: String,
    model: String,
    year: u32,
}

fn get_car_info() -> Car {
    let mut buffer = String::new();

    // Asking for car brand
    print!("What is the brand of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let brand = buffer.trim().to_string();
    buffer.clear();

    // Asking for car model
    print!("What is the model of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    // Asking for car year
    print!("What is the year of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();

    Car { brand, model, year }
}

fn save_car_info(car: &Car) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("user_info.txt")
        .unwrap();

    writeln!(file, "Brand: {}", car.brand).unwrap();
    writeln!(file, "Model: {}", car.model).unwrap();
    writeln!(file, "Year: {}", car.year).unwrap();
}

fn read_car_info() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("\nCar Information from file:");
    println!("{}", content);
}

fn main() {
    let car = get_car_info();
    save_car_info(&car);
    read_car_info();
}
