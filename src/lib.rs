use bevy::prelude::*;
use rand::Rng;
use uuid::Uuid;

pub mod ecs;

const NORMAL_BUTTON: Color = Color::ORANGE;
const HOVERED_BUTTON: Color = Color::GREEN;
const PRESSED_BUTTON: Color = Color::RED;

pub const SCREEN_HEIGHT: f32 = 800.;
pub const SCREEN_WIDTH: f32 = 480.;
pub const SEGMENT_HEIGHT: f32 = 40.;
pub const SEGMENT_WIDTH: f32 = 40.;
pub const CAR_SPEED_FROM: i32 = 80;
pub const CAR_SPEED_TO: i32 = 160;
pub const CAR_WIDTH: f32 = 90.;
pub const CAR_HEIGHT: f32 = 59.;
pub const TRAIN_WIDTH: f32 = 860.;
pub const TRAIN_HEIGHT: f32 = 134.;

pub const LOG_BIG_WIDTH: i32 = 138;
pub const LOG_SMALL_WIDTH: i32 = 84;

const SCROLLING_SPEED_BACKGROUND: f32 = 45.;
const SCROLLING_SPEED_LOGS: f32 = 60.;
const SCROLLING_SPEED_TRAINS: f32 = 800.;
const SCROLLING_SPEED_PLAYER: f32 = 150.;
const SCROLLING_SPEED_EAGLE: f32 = 200.;

// Z coordinates for different components
// determine which component is on
// the top of other component when drawn
const Z_BACKGROUND_ROW: f32 = 1.0;
const Z_BACKGROUND_ROW_GRASS: f32 = 0.5; // for explanation see comment in GameRowBundle::new (for some reason road will overlap...)
const Z_ROW_CHILD_COMPONENT_HEDGE: f32 = 1.0;
const Z_ROW_CHILD_COMPONENT_CAR: f32 = 15.0; // must be more than player
const Z_ROW_CHILD_COMPONENT_TRAIN: f32 = 15.0; // must be more than player
const Z_ROW_CHILD_COMPONENT_EAGLE: f32 = 20.0; // must be more than player, car & train
const Z_ROW_CHILD_COMPONENT_LOG: f32 = 5.0; // must be less than player
const Z_PLAYER: f32 = 10.0;
const Z_SPLASH: f32 = 11.0;
const Z_GRID: f32 = 15.0;
const Z_GAMEOVER: f32 = 20.0;

/// Game states
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    /// initial menu from where player can either exit or start game
    Menu,

    /// player is playing
    InGame,

    /// player has just died
    /// movement and scrolling will be disabled
    /// for couple of seconds to enjoy the scenery
    /// then we will transition to GameOver state
    JustDied,

    /// similar as JustDied but instead of showing eagle
    /// we show splash only before transitioning to GameOver state
    JustDiedInWater,

    /// When this state is entered we display game over caption
    /// and upon pressing space game will exit
    GameOver,
}

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

/// returns boolean mask where true represents gap and one represents populated area of 40px
/// this will be used to populate grass row with bushes randomly
pub fn get_random_row_mask() -> [bool; 12] {
    let mut row_mask: [bool; 12] = [false; 12]; // init with false values

    // generate random 12 elements mask (since screen width is 40*12 pixels)
    // false represents hedge
    // true represents no hedge
    for i in 0..12 {
        row_mask[i] = get_random_float() < 0.01;
    }

    // make at least one gap in hedge mask
    row_mask[get_random_i8(0, 11) as usize] = true;

    // widen each gap by one more gap to left or right
    for i in 0..12 {
        if row_mask[i] == true {
            match i {
                0 => row_mask[1] = true,
                11 => row_mask[10] = true,
                _ => {
                    let left_or_right_idx = if get_random_float() < 0.5 {
                        i - 1
                    } else {
                        i + 1
                    } as usize;
                    row_mask[left_or_right_idx] = true;
                }
            }
        }
    }

    row_mask
}

/// converts current col (0..11 from left to right)
/// to lower and upper (i.e. lower + 40 px) x coordinate
pub fn player_col_to_coords(col: usize) -> (f32, f32) {
    match col {
        6 => (0., 40.),
        7 => (41., 80.),
        8 => (81., 120.),
        9 => (121., 160.),
        10 => (161., 200.),
        11 => (201., 240.),
        5 => (-40., -1.),
        4 => (-80., -41.),
        3 => (-120., -81.),
        2 => (-160., -121.),
        1 => (-200., -161.),
        0 => (-240., -201.),
        _ => (-1., -1.),
    }
}

pub fn player_x_to_player_col(player_x: i32) -> i8 {
    match player_x + 20 {
        0..=40 => 6,
        41..=80 => 7,
        81..=120 => 8,
        121..=160 => 9,
        161..=180 => 10,
        181..=240 => 11,
        -40..=-1 => 5,
        -80..=-41 => 4,
        -120..=-81 => 3,
        -160..=-121 => 2,
        -200..=-161 => 1,
        -240..=-201 => 0,
        _ => -1, // this will never happen since we are controlling player not to cross left/right boundary
    }
}

pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_number() {
        assert_eq!(is_even_number(42), true);
        assert_eq!(is_even_number(37), false);
        assert_eq!(is_even_number(0), true);
        assert_eq!(is_even_number(-10), true);
        assert_eq!(is_even_number(-7), false);
    }
}
