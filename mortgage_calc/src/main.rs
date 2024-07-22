use std::io::{self, Write};

fn main() {
    let mut sale_amount: i32 = 900000; // Initial value (can be removed if not needed)
    let down_payment: i32 = 180000;
    let mut mortgage_amount: i32 = 0;
    let years: i32 = 0; // Placeholder, likely needs user input
    let mut months: i32 = 0; // Placeholder, likely needs user input

    println!("What is your actual sale price? ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse user input to an i32 (assuming valid input)
    sale_amount = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid sale price. Please enter a number.");
            return; // Exit program if input is not a valid number
        }
    };

    mortgage_amount = sale_amount - down_payment;

    while sale_amount >= 0 {
        println!("Sale amount: {}", sale_amount);
        sale_amount -= 10000;
    }

    println!("Mortgage amount: ${}", mortgage_amount);
}
    // Prompt for years and months if needed (modify based on your logic)
    // ...