use rand::Rng;

pub mod ecs;

/// returns a random float from interval <0.,1.)
pub fn get_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// returns a random value from interval <from, to>
pub fn get_random_i8(from: i8, to: i8) -> i8 {
    let mut rng = rand::thread_rng();
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    rng.gen_range(from..to + 1)
}

/// same as get_random_i8. for now keep it simple, no generics
pub fn get_random_i32(from: i32, to: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..to + 1)
}
