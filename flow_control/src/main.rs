fn main() {
    let mut cnt = 0;
    loop {
        println!("Hello");
        if cnt == 10 {
            break;
        }
        cnt += 1;
    }

    let mut cnt = 0;
    while cnt <= 10 {
        println!("Hello");
        cnt += 1;
    }

    for i in [1, 2, 3] {
        println!("Hello, {i}");
    }

    let range = 1..=10;
    for x in range {
        println!("{}", x * x);
    }
}