use rand::Rng;
fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=100);
    println!("{rand_num}");
}