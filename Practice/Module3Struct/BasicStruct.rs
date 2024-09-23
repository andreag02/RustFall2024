#[derive(Debug)]

struct Car {
    color: String,
    make: String,
    year: u16,
}

fn main() {
    let my_car = Car{
        color: "black".to_string(),
        make: "BMW".to_string(),
        year: 2024,
    };
    println!("{:?}", my_car);
}
