use std::fmt::format;

fn main() {
    let c1: char = 'a';
    let c2: char = '@';
    let c3: char = '😊';

    let s1: &str = "Rust";

    let s2: String = String::from("Python");
    let s3: String = "Go".to_string();

    let mut s4: String = String::from("Hello");
    s4.push_str(", Rust!");
    println!("{}", s4);

    // String + &str の順番でないとcompile error
    println!("{}", s4 + "Zig!");

    let s5: String = format!("{}{}", s1, s2);
}
