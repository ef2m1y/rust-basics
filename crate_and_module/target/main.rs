mod test_module;

// use test_module::{sub_module1, sub_module2};
use test_module::*;

fn main() {
    crate::test_module::sub_module1::test_fn1(); // 絶対パス
    test_module::sub_module1::test_fn1(); // 相対パス(self::省略ver.)
    test_module::sub_module2::test_fn1();

    sub_module1::test_fn1();
    sub_module2::test_fn1();

    let s = sub_module1::TestStruct { val1: 10, val2: 20 };
    let t = sub_module1::TestStruct::new(17, 19);

    println!("This is crate_and_module!");
    crate_and_module::say_hello();
}

// cargo run --bin crate_and_module
// cargo run --bin crate_and_module --release
