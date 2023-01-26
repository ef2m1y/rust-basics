fn fizzbuzz_while_if(n: u32) {
    let mut i = 1;
    while i <= n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{i}");
        }
        i += 1;
    }
}

fn fizzbuzz_for_match(n: u32) {
    for i in 1..=n {
        match i % 15 {
            0 => println!("fizzbuzz"),
            3 | 6 | 9 | 12 => println!("fizz"),
            5 | 10 => println!("buzz"),
            _ => println!("{i}"),
            // m => println!("{m}"),
        }
    }
}

fn fizzbuzz_for_match_tuple(n: u32) {}

fn main() {
    // fizzbuzz_while_if(100);
    fizzbuzz_for_match(100);
}
