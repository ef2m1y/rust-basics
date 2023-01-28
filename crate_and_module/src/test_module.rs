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