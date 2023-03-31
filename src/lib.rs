use rand::Rng;

pub mod ecs;

pub const SCREEN_HEIGHT: f32 = 800.;
pub const SCREEN_WIDTH: f32 = 480.;
pub const SEGMENT_HEIGHT: f32 = 40.;

const SCROLLING_SPEED_BACKGROUND: f32 = 45.;
const SCROLLING_SPEED_LOGS: f32 = 60.;
const SCROLLING_SPEED_TRAINS: f32 = 800.;
const SCROLLING_SPEED_CARS: f32 = 200.;

/// returns a random float from interval <0.,1.)
pub fn get_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// same as get_random_i8. for now keep it simple, no generics
/// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
pub fn get_random_i32(from: i32, to: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..to + 1)
}

pub fn get_random_i8(from: i8, to: i8) -> i8 {
    get_random_i32(from as i32, to as i32) as i8
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
