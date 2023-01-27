use std::rc::Rc;

fn main() {
    // Box<i32>: i32型の値を指すBox(ポインタ)型
    // i32型の値がスタックに格納されずヒープに格納される
    let x: Box<i32> = Box::new(1);
    println!("x: {:p}", x);
    println!("*x + 2 = {}", *x + 2);

    // 変数aがhelloという値の所有権を持つ
    let a: Rc<String> = Rc::new("hello".to_string());
    // helloという値の所有者数を確認
    println!("cnt1: {}", Rc::strong_count(&a)); // 1
    {
        // bにもhelloという値の所有権を持たせる
        let b: Rc<String> = Rc::clone(&a);
        println!("a: {:p}", a); // same addr
        println!("b: {:p}", b); // same addr
        println!("cnt2: {}", Rc::strong_count(&a)); // 2
    }
    println!("cnt3: {}", Rc::strong_count(&a)); // 1
}
// 0 (-> ヒープに格納されたhelloが破棄される)


// ＊ポインタ：値の格納されたメモリのアドレスを指す変数
//   ・参照：所有権を持たないポインタ(指している値を借用できる)
//   ・スマートポインタ：指している値を所有できる
//      - Box：値をヒープ領域に格納する
//      - Rc：複数の所有者を持たせつつ(Rustにおいて例外的),
//            安全にメモリを扱うことが可能(参照カウント=0でメモリ解放)
//      - etc.