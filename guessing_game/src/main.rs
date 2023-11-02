// imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

// utility function to print without newline for input
fn print(message: &str) {
    print!("{}", message);
    // doesn't display otherwise
    let _ = io::stdout().flush();
}

// main function
fn main() {
    println!("Guess the number!");

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // don't give the game away
    // println!("The secret number is: {secret_number}");

    // loop until break
    loop {
        print("Input your guess: ");

        // allocate mutable string for guess
        let mut guess = String::new();

        // read a line into `guess`
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // validate guess and convert to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // compare guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
