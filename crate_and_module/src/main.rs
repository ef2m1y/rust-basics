mod test_module {
    pub mod sub_module1 {
        pub fn test_fn1() {
            println!("call test_fn1-1");
        }

        fn test_fn2() {
            println!("call test_fn1-2");
        }
    }
    mod sub_module2 {
        pub fn test_fn1() {
            println!("call test_fn2-1");
        }

        fn test_fn2() {
            println!("call test_fn2-2");
        }
    }
}

use test_module::sub_module1;

fn main() {
    crate::test_module::sub_module1::test_fn1(); // 絶対パス
    test_module::sub_module1::test_fn1(); // 相対パス(self::省略ver.)
    // test_module::sub_module2::test_fn1();
    sub_module1::test_fn1();
}