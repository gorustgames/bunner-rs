use bevy::prelude::*;

pub mod ecs;

#[derive(Debug, Component)]
pub enum MovementDirection {
    LEFT,
    RIGHT,
}
