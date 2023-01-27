struct Rect {
    width: u32,
    height: u32,
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
}
