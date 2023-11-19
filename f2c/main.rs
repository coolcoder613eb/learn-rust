use std::io;
use std::io::Write;

// TODO: put this into a crate later
// utility function to print without newline for input
fn print(message: &str) {
    print!("{}", message);
    // doesn't display otherwise
    let _ = io::stdout().flush();
}

// fahrenheit to celsius
fn f2c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

// celsius to fahrenheit
fn c2f(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

fn main() {
    println!("Convert between Fahrenheit and Celsius");
    println!("Enter a number for either Fahrenheit or Celsius");
    println!("");
    println!("Enter F or C for Fahrenheit or Celsius");
    print("F or C: ");

    // allocate f_or_c
    let mut f_or_c: String = String::new();

    // read a line into f_or_c
    io::stdin()
        .read_line(&mut f_or_c)
        .expect("Failed to read line");

    // interpret f_or_c, storing in is_celsius
    let is_celsius: bool = match f_or_c.trim().to_lowercase().as_str() {
        "c" => true,
        "f" => false,
        _ => {
            println!("Invalid option. Please enter F or C.");
            return;
        }
    };

    // prompt for the number, asking for celsius or fahrenheit depending in is_celsius
    print(
        format!(
            "Enter a number for {}: ",
            (if is_celsius { "Celsius" } else { "Fahrenheit" })
        )
        .as_str(),
    );

    // allocate number
    let mut number: String = String::new();

    // read line into number
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // validate number and convert to float
    let number: f64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // show results, truncating to two decimal places
    if is_celsius {
        let fahrenheit = c2f(number);
        println!(
            "{} Celsius is {} Fahrenheit",
            number,
            format!("{:.2}", fahrenheit)
        );
    } else {
        let celsius = f2c(number);
        println!(
            "{} Fahrenheit is {} Celsius",
            number,
            format!("{:.2}", celsius)
        );
    }
}
