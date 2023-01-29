fn main() {
    // let v1 = vec!["Rust", "Python", "Java"];
    // println!("{:?}", v1);
    // println!("{:?}", v1.as_ptr());
    // println!("{:?}", v1.len());
    // println!("{:?}", v1.capacity());

    // let v2 = vec!["Rust", "Python", "Java"];
    // println!("{:?}", &v2[0]);
    // println!("{:?}", v2.get(0));

    // let mut v3 = vec!["Rust", "Python", "Java"];
    // v3.push("PHP");
    // println!("{:?}", v3);
    // let val = v3.pop();
    // println!("{:?}", val);
    // println!("{:?}", v3);

    // let mut v4 = vec!["Rust", "Python", "Java"];
    // v4.insert(1, "PHP");
    // println!("{:?}", v4);
    // v4.remove(2);
    // println!("{:?}", v4);

    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["PHP", "Go"];
    let v3 = [v1, v2].concat();
    println!("{:?}", v3);

    let (v4, v5) = v3.split_at(2);
    println!("{:?}", v4);
    println!("{:?}", v5);

    let mut v6 = vec![3, 6, 1, 7, 2];
    v6.sort();
    println!("{:?}", v6);
    v6.reverse();
    println!("{:?}", v6);

    // sort keyを指定する方法(もしくはOrd traitの実装でも良い)
    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }
    let mut v7 = vec![
        S { val1: 3, val2: 1 },
        S { val1: 2, val2: 2 },
        S { val1: 1, val2: 3 },
    ];
    // sはv7の各要素
    v7.sort_by_key(|s| s.val1);
    println!("{:?}", v7);

    let v8 = vec![3, 6, 1, 7, 2];
    println!("{:?}", v8.contains(&6)); // true
    println!("{:?}", v8.contains(&5)); // false

    let x = v8.iter().position(|x| *x == 6);
    println!("{:?}", x);
    let y = v8.iter().position(|x| *x == 5);
    println!("{:?}", y);
}
