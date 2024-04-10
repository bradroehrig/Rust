fn main() {
    let mut array = [1,2,3];
    println!("{}", array [0]);
    let mut array2: [i32; 3] = [4,5,6];
    println!("{}", array2[1]);
    array2[0] = 10;
    println!("{}", array2[0]);
    array[1] = 50;
    println!("{}", array[1]);
}
