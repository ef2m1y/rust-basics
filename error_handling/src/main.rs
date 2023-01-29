fn main() {
    // println!("{}", [1, 2, 3][10]); // panic
    // println!("{}", 1 / 0); // panic
    // panic!("This is my panic!");

    println!("{:?}", need_even(1));
    println!("{:?}", need_even(2));
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("I want an even number!"))
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
