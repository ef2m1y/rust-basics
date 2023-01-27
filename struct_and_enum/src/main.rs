struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // self: instance自身を指す -> selfの型は構造体と同じ -> 省略可
    // メソッドでもmoveが起きる -> 共有参照とする
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let height = 5;
    let mut rect = Rect {
        width: 10,
        height,
    };
    println!("width: {}", rect.width);
    println!("height: {}", rect.height);

    rect.height = 10;
    println!("height: {}", rect.height);

    println!("area: {}", rect.area());
}
