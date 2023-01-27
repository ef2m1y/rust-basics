fn main() {
    let mut v1 = vec![1, 2, 3];
    // (fat ptrに含まれる)v1へのptrはv1の0番目の要素のアドレスを指している
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1[0]: {:p}", &v1[0]);

    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity());

    v1.push(4);
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity());

    // // move
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // let v2 = v1;
    // println!("v2 ptr: {:?}", v2.as_ptr());

    let v2 = v1.clone();
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v2 ptr: {:?}", v2.as_ptr());

    let a = 10;
    let b = a;
    println!("a ptr: {:p}", &a);
    println!("b ptr: {:p}", &b);

    // 以降のタプルによる方法は冗長なので参照を用いることになる
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let (s, s1, s2) = concat(s1, s2);
    println!("{s}");
    println!("{s1}");
    println!("{s2}");
}

fn concat(a: String, b: String) -> (String, String, String) {
    let c = format!("{a}, {b}");
    (c, a, b)
}

// ・deep copy: .clone()で実現可能
// ・shallow copy: moveにおいて元所有者の所有権も残存する形式(Rustにはない)
//
// ・moveの例外: Copyトレイト(スタックに保持される型に対して実装可)が実装された型はmoveでなくdeep copyが行われる
//   - 値をcopyしても十分高速であると保証できる
//
// ・Copyトレイトが実装された型の例
//   - 数値型, bool, char, タプル(要素全てCopyの場合)
