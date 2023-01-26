fn main() {
    let x = 0;
    // すべての場合を網羅する
    match x {
        0 => println!("Zero..."),
        1 => {
            println!("One!");
            println!("One!");
        }
        _ => println!("Other!!!"),
    }

    // 1. すべての場合を網羅する
    // 2. 各場合の返り値の型は同じ
    let y = match x {
        0 => 0,
        1 => 10,
        _ => 11111,
    };
}
