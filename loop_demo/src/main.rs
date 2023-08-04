fn main() {
    let mut num = 0;
    'counter: loop {
        println!("count: {}", num);
        let mut decrease = 5;
        loop {
            print!("Decreasing: {} ", decrease);
            if decrease == 4 {
                break;
            }
            if num ==2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }
}
