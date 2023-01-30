use std::{env, fs, fs::File, io, io::prelude::*, io::BufReader};
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // // $ cargo run aaa bbb ccc
    // // ["target/debug/file_operations", "aaa", "bbb", "ccc"]
    // // -> idxを指定し必要な値を取り出せる

    // print!("文字列を入力: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // println!("入力された文字列: {:?}", input);

    // let mut num: i32 = input.trim().parse().unwrap();
    // println!("{:?}", num * 10);

    let mut f = File::open("src/sample1.txt").unwrap();
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    // let contests = fs::read_to_string("src/sample1.txt").unwrap();
    // println!("{}", contents);

    let mut buf_reader = BufReader::new(f);
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    println!("{}", line);
    buf_reader.read_line(&mut line).unwrap();
    println!("{}", line);
}
