use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // $ cargo run aaa bbb ccc 
    // ["target/debug/file_operations", "aaa", "bbb", "ccc"]
    // -> idxを指定し必要な値を取り出せる
}
