use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 1..=5 {
        let rnum:i32 = rng.gen_range(1..=6);
        println!("{}", rnum);
    }
}
