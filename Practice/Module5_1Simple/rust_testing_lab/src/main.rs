fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiplication(c: i32, d: i32) -> i32{
    let mut total = 0;
    for _i in 0..d {
        total = add(total, c);
    }
    total
}

fn exponentiation(base: i32, power: i32) -> f32{

    if power < 0 {
        return 1 as f32 / exponentiation(base, -power);
    }

    let mut total = 1;
    for _i in 0..power{
        total = multiplication(total, base)
    }
    total as f32
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("2 + 5 = {}", multiplication(2, 5));
    println!("2 ** 4 = {}", exponentiation(2, 4));

    let mut x = 5.0;
    let dangerous_value: f32 = 1.0/3.0;
    x = x + dangerous_value;
    println!("5 + 1/3 = {}", x);

    x = x - dangerous_value;
    println!("x - 1/3 = {}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    // assert_eq!
    // assert_me!

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_multiple() {
        // This type of test suite makes testing simpler.
        // However if you were to have thousands on tests in a single function, it would be too difficult to find a bug.

        let test_cases = vec![
            (1, 1, 2),
            (0, 0, 0),
            (-1, 1, 0),
            (100, -50, 50)
        ];
        
        for (a, b, expected) in test_cases {
            assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
        }
    }

    #[test]
    fn test_mult() {
        assert_eq!(multiplication(2,5), 10);
    }

    #[test]
    fn test_mult_negative(){
        assert_eq!(multiplication(-2,1), -2);
    }

    #[test]
    fn test_exponent() {
        assert_eq!(exponentiation(2,4), 16.0);
        assert_eq!(exponentiation(8, 0), 1.0);
        assert_eq!(exponentiation(5, -2), 0.04);
        assert!(f32:: abs(0.333 - exponentiation(3, -1)) < 0.005);
    }
}