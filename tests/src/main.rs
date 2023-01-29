// #[test]
// fn test_sample() {
//     let a = 4 / 2;
//     let b = 2;
//     assert_eq!(a, b);
// }

// 正しくerrorが生じることをtestで確かめる事も重要であり以下取り扱う
fn maybe_panic(flag: bool) {
    if flag == false {
        println!("safe!");
        // 何らかの処理により予想外のpanicが生じた場合
        // testが通ってしまわないようにexpected="部分一致"を追記
        panic!("unexpected!")
    } else {
        panic!("flag is true!!!");
    }
}

// cargo test -> 並行処理(複数スレッド同時実行)
// cargo test -- --test-threads=1 -> 複数testによる環境の共有に対処

// cargo test 関数名(部分一致文字列)

#[cfg(test)] // cargo test時のみこのmoduleをcompile
mod test_module {
    // maybe_panic関数のtest関数
    #[test]
    #[should_panic(expected="flag is true")]
    // panicの発生を判定するassertionはない
    // (代わりにこの属性を付けてpanicの発生を判定する)
    // panicが起きるとtest: ok / panicが起きないとtest: FAILED
    fn test_maybe_panic() {
        super::maybe_panic(false); // 一つ上のスコープなのでsuper::を付ける
    }

    #[test]
    fn test_calc_add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    #[ignore] // testで無視される
    fn test_calc_diff() {
        assert_eq!(1 - 1, 0);
    }
}

fn main() {}
