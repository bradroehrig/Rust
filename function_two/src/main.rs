fn main() {
    println!("{}", gcd (20, 4)); 
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c =a;
            a =b;
            b=c;
        }
        a = a % b;
    }
    b
}
