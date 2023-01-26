fn main() {
    let t1: (i32, bool, f64) = (1, false, 5.0);
    let t2: (bool, f64, i32) = (false, 5.0, 1);
    // debug format
    println!("{:?}", t1);

    println!("{}", t1.0);

    let (x, y, z) = t2;
    println!("{x}");

    let u: () = ();
}