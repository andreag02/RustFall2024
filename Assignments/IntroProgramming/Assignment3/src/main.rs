fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess > secret{
        1
    }
    else{
        -1
    }
}
fn main() {
    let secret: i32 = 15;

    let mut num_attempts: i32 = 0;
    let mut guess: i32 = 15;

    loop {
        num_attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("You guessed correctly! The secret number was {}.", secret);
            break;
        }
        else if result == 1 {
            println!("You guessed too high! Try again.");
        }
        else {
            println!("You guessed too low! Try again.");
        }
    }
    println!("It took you {} attempts to guess correctly", num_attempts);
}
