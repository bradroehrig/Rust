fn main() {
    // Creating an empty vector of integers
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to the vector using `push` method
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    numbers.push(40);

    // Accessing elements in the vector using indexing
    println!("Third element: {}", numbers[2]); // Output: Third element: 30

    // Iterating over the vector using a for loop
    for number in &numbers {
        println!("{}", number);
    }

    // Updating elements in the vector using indexing
    numbers[1] = 25;

    // Removing elements from the vector using `pop` method
    let removed_element = numbers.pop();
    println!("Removed element: {:?}", removed_element); // Output: Removed element: Some(40)

    // Checking the length of the vector
    println!("Vector length: {}", numbers.len()); // Output: Vector length: 3
}
