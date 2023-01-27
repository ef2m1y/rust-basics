fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s = concat(&s1, &s2);
    println!("{s}");
    println!("{s1}");
    println!("{s2}");
}

fn concat(a: &String, b: &String) -> String {
    let c = format!("{a}, {b}");
    c
}
