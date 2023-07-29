fn main () {
    let mut s1 = String::from("Hello");
    calculate_length(&mut s1);
    println!("{}", s1);
}

fn calculate_length(s: &mut String) {
    s.push_str(" world!")
}