use rand::Rng;

pub mod ecs;

/// returns a random float from interval <0.,1.)
fn get_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// returns a random value from interval <from, to>
fn get_random_int(from: i8, to: i8) -> i8 {
    let mut rng = rand::thread_rng();
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    rng.gen_range(from..to + 1)
}
