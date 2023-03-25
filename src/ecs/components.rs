use bevy::prelude::*;

pub mod background_row;
pub mod log;
pub mod train;

#[derive(Debug, Component, PartialEq)]
pub enum MovementDirection {
    LEFT,
    RIGHT,
}

/// This timer is used to despawn  entity and its children
/// at delayed moment. This is to prevent abrupt disappearance of
/// sprites like trains which are bound to bottom row of respective
/// background sprite (e.g. rail1.jpg) and occupy space across multiple
/// upper rows (rail2, rail3). Since rail1 scrolls off couple of seconds sooner
/// than rail3, its immediate de-spawning would make train disappear in unnatural way.
/// Delaying this for couple of seconds by this timer will solve this issue.
#[derive(Component)]
pub struct DespawnEntityTimer {
    pub timer: Timer,
}

impl DespawnEntityTimer {
    pub fn new(delay_sec: f32) -> Self {
        DespawnEntityTimer {
            timer: Timer::from_seconds(delay_sec, false),
        }
    }
}
