#[derive(Debug)]

struct Car {
    color: String,
    make: String,
    year: u16,
}

impl Car {
    fn new(color: String, make: String, year: u16) -> Car {
        Car {
            color: color,
            make: make,
            year: year,
        }
    }

    fn honk_honk(&self){
        println!("My car with color {} honk honk", self.color);
    }

    fn upgrade(&mut self, year: u16){
        self.year = year;
    }
}

fn main() {
    let mut my_car: Car = Car::new(
        "black".to_string(),
        "BMW".to_string(),
        2024,
    );
    println!("{:?}", my_car);

    my_car.honk_honk();

    my_car.upgrade(2025);
    println!("{:?}", my_car);
}
