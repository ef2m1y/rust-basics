fn say_foo() {
    println!("foo!!!");
}

fn mul(a: i32, b: i32) ->i32 {
    a * b
}

fn main() {
    say_foo();
    println!("{}", mul(9, 9));

    let x: i32 = mul(3, 5);
    
    let u: () = say_foo();
}
