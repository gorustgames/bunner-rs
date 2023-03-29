use bevy::prelude::*;

pub mod background_row;
pub mod log;
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
pub struct DelayerTrainTimer {
    pub timer: Timer,
}

impl DelayerTrainTimer {
    pub fn new(delay_sec: f32) -> Self {
        DelayerTrainTimer {
            timer: Timer::from_seconds(delay_sec, false),
        }
    }
}
