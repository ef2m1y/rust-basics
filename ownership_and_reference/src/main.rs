fn main() {
    let x1: Vec<i32> = vec![1, 2, 3];
    let x2: &Vec<i32> = &x1;
    // let x3: &&Vec<i32> = &x2;

    // same
    println!("{:?}", x1);
    println!("x1 ptr: {:?}", x1.as_ptr());
    println!("{:?}", x2);
    println!("x2 ptr: {:?}", x2.as_ptr());
}
