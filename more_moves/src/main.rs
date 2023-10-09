fn main() {
    // let s = String::from("takes");
    // takes_ownership(s);

    // let val = 1;
    // println!();
    
    // make_copy(val);
    // println!();

    // let str1: String = give_ownership();
    // println!("{}", str1);
    // println!();

    // let str3: String = take_and_give(str1);

    // if (true) {
    //     let str4 = str3;
    // }else{ 
    //     let str5 = str3;
    // }

    // println!("{}", str3);
    
//     let mut str1 = String::from("Tyler");
//     let mut str2: String;
    
//     loop {
//         str2 = str1;
//     }

// }

// fn takes_ownership(s: String){
//     let strin = s;
//     print!("{}", strin);
// }

// fn make_copy(one: i32){
//     let val1 = one;
//     print!("{}", val1);
// }

// fn give_ownership() -> String {
//     "given".to_string()
// }

// fn take_and_give(str2: String ) -> String {
//     str2

let mut s = String::from("Hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(some_string: &mut String){
    some_string.push_str(" world!");
}
