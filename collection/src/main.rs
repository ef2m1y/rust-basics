fn main() {
    let v1 = vec!["Rust", "Python", "Java"];
    println!("{:?}", v1);
    println!("{:?}", v1.as_ptr());
    println!("{:?}", v1.len());
    println!("{:?}", v1.capacity());

    let v2 = vec!["Rust", "Python", "Java"];
    println!("{:?}", &v2[0]);
    println!("{:?}", v2.get(0));

    let mut v3 = vec!["Rust", "Python", "Java"];
    v3.push("PHP");
    println!("{:?}", v3);
    let val = v3.pop();
    println!("{:?}", val);
    println!("{:?}", v3);

    let mut v4 = vec!["Rust", "Python", "Java"];
    v4.insert(1, "PHP");
    println!("{:?}", v4);
    v4.remove(2);
    println!("{:?}", v4);
}
