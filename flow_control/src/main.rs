fn main() {
    let x = 1;
    println!("{x}");
    {
        let x = 999;
        println!("{x}");
    }
    println!("{x}");

    let y = { 100 };
    let z: () = {};
}
