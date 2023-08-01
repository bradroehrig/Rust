fn main() {
    // Define an array of integers
    let numbers = [10, 20, 30, 40, 50];

    // Access individual elements using array indexing
    let first_number = numbers[0];
    let third_number = numbers[2];
    let fifth_number = numbers[4];

    println!("First number: {}", first_number);
    println!("Third number: {}", third_number);
    println!("Fifth number: {}", fifth_number);

    // You can also modify elements in the array using indexing
    let mut mutable_numbers = [1, 2, 3];
    mutable_numbers[1] = 5;
    println!("Modified array: {:?}", mutable_numbers);
}
