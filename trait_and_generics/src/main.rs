use trait_and_generics::sample_trait::{Shape, Rectangle, Circle, double_area};
// 型をimportしただけでは型に実装されたトレイトのメソッドは使用不可能
// -> Shapeトレイトもimportする

fn main() {
    let rect = Rectangle {
        width: 5.0,
        height: 2.0,
    };
    let circle = Circle {
        radius: 10.0,
    };

    println!("Rectangle area is: {}", rect.calc_area());
    println!("Rectangle parimeter is: {}", rect.calc_perimeter());
    Rectangle::do_something();
    println!("Circle area is: {}", circle.calc_area());
    println!("Circle parimeter is: {}", circle.calc_perimeter());
    Circle::do_something();

    println!("Rectangle default: {}", rect.default_something());
    println!("Circle default: {}", circle.default_something());

    println!("Rectangle double area: {}", double_area(&rect));
    println!("Circle double area: {}", double_area(&circle));
}