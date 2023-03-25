use bevy::prelude::*;

pub mod background_row;
pub mod log;
pub mod train;

#[derive(Debug, Component, PartialEq)]
pub enum MovementDirection {
    LEFT,
    RIGHT,
}
