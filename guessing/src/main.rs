use std::io;

fn main() {
    let secret_number = 7; // For simplicity, the secret number is fixed

    loop {
        println!("Guess the number between 1 and 10:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line"); //reads a line of user input into the mutable String variable guess.

        let guess: u32 = match guess.trim().parse() {  //Used to convert user input into a specific type after removing any extraneous whitespace.
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess == secret_number {
            println!("Congratulations! You've guessed the correct number.");
            break;
        } else {
            println!("Try again!");
        }
    }
}
