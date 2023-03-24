use bevy::prelude::*;

pub mod background_row;
pub mod log;

#[derive(Debug, Component)]
pub enum MovementDirection {
    LEFT,
    RIGHT,
}
