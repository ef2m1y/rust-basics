pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64; // method
        fn calc_perimeter(&self) -> f64; // method
        fn default_something(&self) -> &str {
            "This is default method!"
        } // (default) method
        fn do_something(); // associated constant
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,       
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }

        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn default_something(&self) -> &str {
            "This is (overwritten) Rectangle method!"
        }

        fn do_something() {
            println!("This is Rectangle function!");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("This is Circle function!");
        }
    }
}

// trait: 
// - 共通な振舞いの仕様を定義ためのもの
// - メソッド/関連関数のsignatureを定義するだけ, 処理の中身は実装しない
// - 任意の型に紐づける(=トレイトを実装する)事で, 
//   その型に対してトレイトに定義されたメソッドの実装を強制する