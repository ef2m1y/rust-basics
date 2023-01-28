pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something();
    }
}

// trait: 
// - 共通な振舞いの仕様を定義ためのもの
// - メソッド/関連関数のsignatureを定義するだけ, 処理の中身は実装しない
// - 任意の型に紐づける事(=トレイトを実装する)で, 
//   その型に対してトレイトに定義されたメソッドの実装を強制する