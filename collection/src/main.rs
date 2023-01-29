use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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

    // let v1 = vec!["Rust", "Python", "Java"];
    // let v2 = vec!["PHP", "Go"];
    // let v3 = [v1, v2].concat();
    // println!("{:?}", v3);

    // let (v4, v5) = v3.split_at(2);
    // println!("{:?}", v4);
    // println!("{:?}", v5);

    // let mut v6 = vec![3, 6, 1, 7, 2];
    // v6.sort();
    // println!("{:?}", v6);
    // v6.reverse();
    // println!("{:?}", v6);

    // // sort keyを指定する方法(もしくはOrd traitの実装でも良い)
    // #[derive(Debug)]
    // struct S {
    //     val1: i32,
    //     val2: i32,
    // }
    // let mut v7 = vec![
    //     S { val1: 3, val2: 1 },
    //     S { val1: 2, val2: 2 },
    //     S { val1: 1, val2: 3 },
    // ];
    // // sはv7の各要素
    // v7.sort_by_key(|s| s.val1);
    // println!("{:?}", v7);

    // let v8 = vec![3, 6, 1, 7, 2];
    // println!("{:?}", v8.contains(&6)); // true
    // println!("{:?}", v8.contains(&5)); // false

    // let x = v8.iter().position(|x| *x == 6);
    // println!("{:?}", x);
    // let y = v8.iter().position(|x| *x == 5);
    // println!("{:?}", y);

    // // 二重終端キュー
    // let mut q = VecDeque::new();
    // // let q = VecDeque::from(vec![1, 2, 3]);

    // q.push_back(1);
    // q.push_back(2);
    // q.push_back(3);
    // println!("{:?}", q);

    // println!("{:?}", q.pop_front());
    // println!("{:?}", q);

    // // 優先度付きキュー(ex. BinaryHeap)
    // let mut bh = BinaryHeap::new();
    // bh.push(1);
    // bh.push(10);
    // bh.push(20);
    // bh.push(2);
    // println!("{:?}", bh); // [20, 2, 10, 1]
    // println!("{:?}", bh.pop());
    // println!("{:?}", bh); // [10, 2, 1]

    // let mut map = HashMap::new();
    // map.insert("Japan", 11);
    // map.insert("USA", 3);
    // map.insert("China", 1);
    // map.insert("India", 2);
    // println!("{:?}", map);

    // map.insert("Japan", 10); // overwrite
    // println!("{:?}", map);

    // println!("{:?}", map.get("USA"));
    // println!("{:?}", map.remove("India"));
    // println!("{:?}", map);

    // for (k, v) in &map {
    //     println!("{:?}: {:?}", k, v);
    // }

    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    println!("{:?}", set1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    println!("{:?}", set1);
    println!("{:?}", set1.contains(&2));
    println!("{:?}", set1.remove(&2));
    println!("set1: {:?}", set1);

    let mut set2 = HashSet::new();
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(5);
    println!("set2: {:?}", set2);

    let set3 = &set1 | &set2;
    println!("set3: {:?}", set3);
    let set4 = &set1 & &set2;
    println!("set4: {:?}", set4);
    let set5 = &set1 - &set2;
    println!("set5: {:?}", set5);
    let set6 = &set1 ^ &set2;
    println!("set6: {:?}", set6);
}
