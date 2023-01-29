fn main() {
    // let c1 = |x: i32| x + 1;
    // println!("{}", c1(10));

    // // クロージャはクロージャ宣言時の自由変数mの状態をクロージャ内に閉じ込める
    // // -> 環境をキャプチャする -> closure
    // let m = 10;
    // let c2 = |x: i32| x + m;
    // println!("{}", c2(3));

    // let m = 20;
    // println!("{}", c2(3));

    // // 自由変数が閉じ込められる際, 自由変数が束縛している値がCopyトレイトを
    // // 実装しているならばcopyが作成され, 実装されていない場合は参照を受け取る

    // // (ベクタはCopyトレイトが実装されていないため, 参照を渡す)
    // let v1 = vec![1, 2, 3];
    // let c3 = || {
    //     println!("{:?}", v1);
    // };
    // c3();
    // println!("{:?}", v1); // ok

    // // (強制的にmoveを起こすことが可能)
    // let v2 = vec![1, 2, 3];
    // let c4 = move || {
    //     println!("{:?}", v2);
    // };
    // c3();
    // // println!("{:?}", v2); // err

    // let v = vec![1, 2, 3, 4, 5];
    // let v1_iter = v.iter();
    // for x in v1_iter {
    //     println!("{}", x); // auto dereference
    // }

    // // iteratorは内部に今が何番目かの情報を有する -> next毎に内部情報更新
    // // -> nextを呼び出すiterator(=v2_iter)はmutである必要がある
    // let mut v2_iter = v.iter();
    // // [v.iter() -> vの共有参照へのiterator] -> nextで得られる値は共有参照
    // println!("{:?}", v2_iter.next());
    // println!("{:?}", v2_iter.next());
    // println!("{:?}", v2_iter.next()); // "iteratorを消費する"
    // println!("{:?}", v2_iter.next());
    // println!("{:?}", v2_iter.next());
    // println!("{:?}", v2_iter.next()); // None
    // println!("{:?}", v2_iter.next()); // None

    // let mut v = vec![1, 2, 3, 4, 5];
    // let mut v2_iter = v.iter_mut();
    // // [v.iter_mut() -> vの可変参照へのiterator] -> nextで得られる値は可変参照
}

// 全てのiteratorは次のようなiterator traitを実装している
// pub trait Iterator {
//     type Item; // 関連型: iteratorが扱う型を表す

//     fn next(&mut self) -> Option<Self::Item>;

//     // default method
//     // default method
//     // ...
// }