fn main() {
    let x = 30;
    if x > 0 {
        println!("hoge!hoge!");
    }
    if x >= 10 && x <= 15 {
        println!("geho!geho!");
    }

    // 1. すべての場合を網羅する
    // 2. 各場合の返り値の方は同じ
    let y = if x > 0 {
        x
    } else {
        0
    };
}