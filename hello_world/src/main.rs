use std::io;

// Example function to call
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    println!("Please enter your name:");

    // Read user input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim leading and trailing whitespaces
    let name = input.trim();

    // Call the function
    greet(name);
}