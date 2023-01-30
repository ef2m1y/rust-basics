use std::{env, fs, fs::File, fs::OpenOptions, io, io::prelude::*, io::BufReader};
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

    // let mut f = File::open("src/sample1.txt").unwrap();
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    // let contests = fs::read_to_string("src/sample1.txt").unwrap();
    // println!("{}", contents);

    // let mut buf_reader = BufReader::new(f);
    // let mut line = String::new();
    // buf_reader.read_line(&mut line).unwrap();
    // println!("{}", line);
    // buf_reader.read_line(&mut line).unwrap();
    // println!("{}", line);

    // let lines = buf_reader.lines();
    // for l in lines {
    //     println!("{}", l.unwrap());
    // }

    // let mut bytes = Vec::new();
    // f.read_to_end(&mut bytes).unwrap();
    // println!("{:?}", bytes);
    // println!("{:?}", String::from_utf8(bytes).unwrap());

    // let mut f1 = File::create("src/sample2.txt").unwrap();
    // let bytes = b"write examples!\n";
    // // println!("{:?}", bytes);
    // f1.write_all(bytes).unwrap();
    // // $ cat src/sample2.txt -> write examples!

    // let mut f2 = File::create("src/sample3.txt").unwrap();
    // writeln!(f2, "Hello, {}!", "Rust").unwrap();
    // // $ cat src/sample3.txt -> Hello, Rust!

    let f1 = OpenOptions::new()
    .append(true) // 上書きでなく続けて書く
    .open("src/sapmle1.txt").unwrap();

    // 元々その名前のファイルがなかった場合にのみ生成する
    let f2 = OpenOptions::new()
    .write(true) // 上書き
    .create_new(true) // 元々ファイルがあったならerrを吐く
    .open("src/sample2.txt").unwrap();
}
