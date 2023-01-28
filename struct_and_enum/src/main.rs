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
    // 型関連関数(&selfなし), コンストラクタ, Self(of Rect)
    fn new(width: u32, height: u32) -> Self {
        // 省略形
        Rect { width, height }
    }
}

enum Shape {
    Circle,
    Square(u32),
    Triangle { base: u32, height: u32 },
}

impl Shape {
    fn samp_method(&self) {
        println!("call samp_method!");
    }
}

fn main() {
    // let height = 5;
    // let mut rect = Rect {
    //     width: 10,
    //     height,
    // };
    let mut rect = Rect::new(10, 5);

    println!("width: {}", rect.width);
    println!("height: {}", rect.height);

    rect.height = 10;
    println!("height: {}", rect.height);

    println!("area: {}", rect.area());

    let c = Shape::Circle;
    let s = Shape::Square(1);
    let t = Shape::Triangle { base: 10, height: 5 };

    c.samp_method();
    s.samp_method();
    t.samp_method();
}

// instance名.method名()
// struct名.型関連関数名()