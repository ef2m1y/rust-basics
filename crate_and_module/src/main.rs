mod test_module;

use test_module::sub_module1;

fn main() {
    crate::test_module::sub_module1::test_fn1(); // 絶対パス
    test_module::sub_module1::test_fn1(); // 相対パス(self::省略ver.)
    test_module::sub_module2::test_fn1();
    
    sub_module1::test_fn1();
}