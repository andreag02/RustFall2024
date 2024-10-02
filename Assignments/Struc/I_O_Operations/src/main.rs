use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

struct Car{
    make: String,
    model: String,
    year: u32,
}

fn reading_from_console() -> Car{
    let mut buffer = String::new();

    print!("Enter the make of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("Enter the model of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("Enter the year of your car: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();
    buffer.clear();

    let users_car = Car{make, model, year};
    println!();

    users_car
}

fn create_and_write_file(car: &Car) {
    let mut file = File::create("user_info.txt").unwrap();
    println!("Writing to new file...\n");
    writeln!(file, "Make: {}", car.make).unwrap();
    writeln!(file, "Model: {}", car.model).unwrap();
    writeln!(file, "Year: {}", car.year).unwrap();
}

fn read_from_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn main(){
    let users_car = reading_from_console();
    create_and_write_file(&users_car);
    read_from_file();
}