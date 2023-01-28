// use trait_and_generics::sample_trait::{Shape, Rectangle, Circle, double_area};
// 型をimportしただけでは型に実装されたトレイトのメソッドは使用不可能
// -> Shapeトレイトもimportする

use std::fmt::{Debug, Display};

fn main() {
    // let rect = Rectangle {
    //     width: 5.0,
    //     height: 2.0,
    // };
    // let circle = Circle {
    //     radius: 10.0,
    // };

    // println!("Rectangle area is: {}", rect.calc_area());
    // println!("Rectangle parimeter is: {}", rect.calc_perimeter());
    // Rectangle::do_something();
    // println!("Circle area is: {}", circle.calc_area());
    // println!("Circle parimeter is: {}", circle.calc_perimeter());
    // Circle::do_something();

    // println!("Rectangle default: {}", rect.default_something());
    // println!("Circle default: {}", circle.default_something());

    // println!("Rectangle double area: {}", double_area(&rect));
    // println!("Circle double area: {}", double_area(&circle));

    // println!("{:?}", (1, 2, 3));
    // // impl Debug for S よりも手軽
    // #[derive(Debug, PartialEq)]
    // struct S {
    //     val1: i32,
    //     val2: i32,
    // }
    // // the trait `Debug` is not implemented for `S`
    // println!("{:?}", S { val1: 1, val2: 2 });

    // let s1 = S {
    //     val1: 1,
    //     val2: 2,
    // };
    // let s2 = S {
    //     val1: 1,
    //     val2: 3,
    // };
    // // binary operation `==` cannot be applied to type `S`
    // // an implementation of `PartialEq<_>` might be missing for `S`
    // println!("{}", s1 == s2);

    // println!("{}", max(1, 2));
    // println!("{}", max(1.1, 2.1));
    // println!("{}", max("a", "c"));

    let p1 = Point { x: 2, y: 1 };
    let p2 = Point { x: 2.0, y: 4.0 };
    let p3 = Point { x: "y", y: "x" };
    println!("p1.max: {:?}", p1.max());
    println!("p2.max: {:?}", p2.max());
    println!("p3.max: {:?}", p3.max());

    p1.print_arg("test");
    p1.print_arg(true);

    println!("p1.min: {:?}", p1.min());
    // println!("p2.min: {:?}", p2.min()); // err
}

// fn max<T>(a: T, b: T) -> T
// where
//     T: PartialOrd + Debug,
// {
//     if a >= b {
//         a
//     } else {
//         b
//     }
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Debug> Point<T> {
    // T型: moveが起こる型である可能性アリ -> 返り値: 共有参照にしておく
    fn max(&self) -> &T {
        if (*self).x >= self.y {
            &(self.x)
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x: {:?}", self.x);
        println!("val: {}", val);
    }
}

impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}