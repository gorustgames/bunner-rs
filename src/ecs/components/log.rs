use crate::ecs::components::MovementDirection;
use crate::Z_ROW_CHILD_COMPONENT_LOG;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt;

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

impl fmt::Debug for LogBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogBundle")
            .field("x", &self.sprite_bundle.transform.translation.x)
            .field("y", &self.sprite_bundle.transform.translation.y)
            .field("size", &self.log_size)
            .field("direction", &self.direction)
            .finish()
    }
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
                transform: Transform::from_xyz(x, y, Z_ROW_CHILD_COMPONENT_LOG), // z=1 since log is child component drawn over its parent
                ..default()
            },
            direction,
            log_size,
        }
    }

    /// spawn log bundle and add it as child to its parent entity (respective water row)
    pub fn spawn_log(self, commands: &mut Commands, parent_entity: Entity) {
        let log = commands.spawn_bundle(self).id();

        commands.entity(parent_entity).add_child(log);
    }
}
