fn main() {
    // println!("{}", [1, 2, 3][10]); // panic
    // println!("{}", 1 / 0); // panic
    // panic!("This is my panic!");

    // println!("{:?}", need_even(1));
    // println!("{:?}", need_even(2));

    // let x = match need_even(2) {
    //     Ok(val) => val,
    //     Err(err) => {
    //         println!("{}", err);
    //         panic!();
    //     }
    // };
    // println!("{x}");

    // let s = need_even(1);
    // println!("{}", s.is_ok());
    // println!("{}", s.is_err());
    
    // moveが起きるのでコメントアウト
    // println!("{:?}", s.ok());
    // println!("{:?}", s.err());

    // println!("{}", s.unwrap_or(100));

    // println!("{}", s.unwrap());
    // println!("{}", s.expect("expect method!"));

    println!("{:?}", double_even(2));
}

fn double_even(b: i32) -> Result<i32, String> {
    // match need_even(b) {
    //     Ok(val) => Ok(val * 2),
    //     Err(err) => Err(err),
    // }
    
    // [?構文]
    // - need_even(b)がOk(val) -> need_even(b)?の評価値はval -> xに格納
    // - need_even(b)がErr(err) -> 即時に, double_even(b)の評価値としてErr(err)が返される
    let x = need_even(b)?;
    Ok(x * 2)
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
