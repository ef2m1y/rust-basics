fn f<'a>(p: &'a i32) {
    // lifetimeを推論せずに書く場合ライフタイムパラメータを記述する
    // 'aや'bのように記述する(複数あれば並べて書く)
    // tick aと発音し，"任意の生存期間'aを持つ参照"の意
}

fn main() {}
