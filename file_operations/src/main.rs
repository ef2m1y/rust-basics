use std::{env, io};
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // // $ cargo run aaa bbb ccc
    // // ["target/debug/file_operations", "aaa", "bbb", "ccc"]
    // // -> idxを指定し必要な値を取り出せる

    print!("文字列を入力: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("入力された文字列: {:?}", input);

    let mut num: i32 = input.trim().parse().unwrap();
    println!("{:?}", num * 10);
}
