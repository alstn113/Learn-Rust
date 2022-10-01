use rand::{thread_rng, Rng};

fn main() {
    for _ in 0..5 {
        let random_u16 = rand::random::<u16>();
        println!("{random_u16}");
    }

    let mut rng = thread_rng();
    for _ in 0..5 {
        println!("{}", rng.gen_range(1..=11)); // gen_range(1, 10)
        println!("{}", rng.gen_range('a'..'z'));
    }
}
