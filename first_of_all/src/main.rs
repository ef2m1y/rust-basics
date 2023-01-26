fn main() {
    let v1: Vec<i32> = vec![10, 20, 30];
    let v2: Vec<i32> = vec![1; 100];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    println!("{:?}", v3);

    let z: Option<i32> = v3.pop();
    println!("{:?}", z);
    println!("{:?}", v3);

    let x: i32 = v3[0];
    let y: Option<&i32> = v3.get(1);
    println!("{}", x);
    println!("{:?}", y); // Some(2)

    let n: Option<&i32> = v3.get(1000);
    println!("{:?}", n); // None

    let s: &[i32] = &v3[..];
}
