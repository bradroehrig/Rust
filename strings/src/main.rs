fn main() {
    let name = String::from("Brad");
    let course = "Rust";  // No need for `to_string()` here
    println!("Hello, my name is {} and I'm learning {}.", name, course);
    println!("{}", name);
    println!("{}", course);
    let new_name = name.replace("Brad", "Bradley");
    println!("{}", new_name);

    let str1 = "Hello";
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    println!("{}", "ONE".to_lowercase() == "one");

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust)

}
