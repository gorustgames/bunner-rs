use crate::ecs::components::MovementDirection;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Debug, PartialEq, Component)]
pub enum LogSize {
    SMALL,
    BIG,
}

#[derive(Bundle)]
pub struct LogBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    direction: MovementDirection,
    log_size: LogSize,
}

impl LogBundle {
    pub fn new(
        direction: MovementDirection,
        log_size: LogSize,
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        LogBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(if log_size == LogSize::BIG {
                    "images/log1.png"
                } else {
                    "images/log0.png"
                }),
                // use global transform (instead of transform) to position child entity globally to window, rather than relative to parent entity!
                //global_transform: GlobalTransform::from_xyz(x, y, 1.), // z=1 since log is child component drawn over its parent
                transform: Transform::from_xyz(x, y, 1.), // z=1 since log is child component drawn over its parent
                ..default()
            },
            direction,
            log_size,
        }
    }
}
