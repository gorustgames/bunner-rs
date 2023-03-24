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

pub fn is_even_number<T>(num: T) -> bool
where
    T: std::ops::Rem<Output = T> + From<i8> + PartialEq,
{
    num % T::from(2i8) == T::from(0i8)
}

pub fn is_odd_number<T>(num: T) -> bool
where
    T: std::ops::Rem<Output = T> + From<i8> + PartialEq,
{
    !is_even_number(num)
}

#[cfg(test)]
mod tests {
    use super::is_even_number;

    #[test]
    fn test_is_even_number() {
        assert_eq!(is_even_number(42), true);
        assert_eq!(is_even_number(37), false);
        assert_eq!(is_even_number(0), true);
        assert_eq!(is_even_number(-10), true);
        assert_eq!(is_even_number(-7), false);
    }
}
