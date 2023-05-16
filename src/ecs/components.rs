use bevy::prelude::*;

pub mod background_row;
pub mod bush;
pub mod car;
pub mod debug_text;
pub mod log;
pub mod player;
pub mod train;

#[derive(Debug, Component, PartialEq, Clone)]
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

/// Marker component we are inserting to delayed train (in timer system) to indicate
/// timer is finished and train should be displayed. Once this component is added
/// respective system can pick up the train and start moving it
#[derive(Component)]
pub struct DelayedTrainReadyToBeDisplayedMarker;

#[derive(Component)]
pub struct TrainTimer {
    pub timer: Timer,
}

impl TrainTimer {
    pub fn new(delay_sec: f32) -> Self {
        TrainTimer {
            timer: Timer::from_seconds(delay_sec, false),
        }
    }
}

/// same 3 components to support delayed spawning of the cars
#[derive(Component)]
pub struct DelayedCarReadyToBeDisplayedMarker;

#[derive(Component)]
pub struct CarTimer {
    pub timer: Timer,
}

impl CarTimer {
    pub fn new(delay_sec: f32) -> Self {
        CarTimer {
            timer: Timer::from_seconds(delay_sec, false),
        }
    }
}

#[derive(Component)]
pub struct ButtonExitMarker;

#[derive(Component)]
pub struct ButtonPlayMarker;
