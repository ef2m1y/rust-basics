use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    fs::{File, OpenOptions},
    io,
    io::prelude::*,
    io::BufReader,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

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

    // let f1 = OpenOptions::new()
    // .append(true) // 上書きでなく続けて書く
    // .open("src/sapmle1.txt").unwrap();

    // // 元々その名前のファイルがなかった場合にのみ生成する
    // let f2 = OpenOptions::new()
    // .write(true) // 上書き
    // .create_new(true) // 元々ファイルがあったならerrを吐く
    // .open("src/sample2.txt").unwrap();

    // let p = Person {
    //     name: String::from("Json Taro"),
    //     age: 55,
    //     phones: vec![String::from("080-XXXX-XXXX"), String::from("090-XXXX-XXXX")],
    // };
    // let json_data = serde_json::to_string_pretty(&p).unwrap();
    // let mut f = File::create("src/sample.json").unwrap();
    // writeln!(f, "{}", json_data).unwrap();

    // let f = File::open("src/sample.json").unwrap();
    // let buf_reader = BufReader::new(f);
    // let data: Person = serde_json::from_reader(buf_reader).unwrap();
    // println!("{:?}", data);

    // let path = Path::new("src");
    // println!("{:?}", path.exists());
    // println!("{:?}", path.is_dir());
    // println!("{:?}", path.is_file());
    // println!("{:?}", path.file_name());

    // let mut path_buf = PathBuf::from("src");
    // path_buf.push("sample1.txt");
    // println!("{:?}", path_buf);
    // path_buf.set_file_name("path.txt");
    // println!("{:?}", path_buf);
    // path_buf.pop();
    // println!("{:?}", path_buf);

    // fs::create_dir("src/test1").unwrap();
    // fs::create_dir_all("src/test2/test2-1/test2-1-1").unwrap();
    // fs::remove_dir("src/test1").unwrap();
    // fs::remove_dir_all("src/test2").unwrap();
    // fs::remove_file("src/sample1.txt").unwrap();
    // fs::copy("src/sample2.txt", "src/sample3.txt").unwrap();
    // fs::create_dir("src/test1").unwrap();
    // fs::rename("src/sample3.txt", "src/test1/sample3.txt").unwrap();

    // $ ls -l src/sample2.txt -> 0o644 (before)
    let mut permissions = fs::metadata("src/sample2.txt").unwrap().permissions();
    permissions.set_mode(0o600);
    fs::set_permissions("src/sample2.txt", permissions).unwrap();
    // $ ls -l src/sample2.txt -> 0o600 (after)
}

// // featuresフラグにderiveを指定しない場合は次の記述が出来ず
// // Serialize/DeserializeトレイトをPerson構造体に実装する必要が出てくる
// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }
