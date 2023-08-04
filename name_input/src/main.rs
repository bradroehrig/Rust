use std::io;

fn main() {
    // Prompt the user for input
    println!("Please enter your name:");

    // Create a new mutable String to store the user's input
    let mut input = String::new();

    // Read the user's input into the String
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character at the end of the input
    let name = input.trim();

    // Print a greeting message using the user's input
    println!("Hello, {}! Nice to meet you!", name);
}
