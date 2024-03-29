use crate::ecs::components::MovementDirection;
use crate::{get_uuid, LOG_BIG_WIDTH, LOG_SMALL_WIDTH, Z_ROW_CHILD_COMPONENT_LOG};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt;

#[derive(Debug, PartialEq, Component, Clone)]
pub enum LogSize {
    SMALL,
    BIG,
}

impl Into<f32> for &LogSize {
    fn into(self) -> f32 {
        match self {
            LogSize::SMALL => LOG_SMALL_WIDTH as f32,
            LogSize::BIG => LOG_BIG_WIDTH as f32,
        }
    }
}

#[derive(Component)]
pub struct LogBundleUuid(String);

impl LogBundleUuid {
    pub fn get_uuid(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Bundle)]
pub struct LogBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    direction: MovementDirection,
    log_size: LogSize,
    uuid: LogBundleUuid,
}

impl fmt::Debug for LogBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogBundle")
            .field("x", &self.sprite_bundle.transform.translation.x)
            .field("y", &self.sprite_bundle.transform.translation.y)
            .field("gx", &self.sprite_bundle.global_transform.translation.x)
            .field("gy", &self.sprite_bundle.global_transform.translation.y)
            .field("size", &self.log_size)
            .field("direction", &self.direction)
            .field("uuid", &self.uuid.0)
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
            uuid: LogBundleUuid(get_uuid()),
        }
    }

    /// spawn log bundle and add it as child to its parent entity (respective water row)
    pub fn spawn_log(self, commands: &mut Commands, parent_entity: Entity) {
        let log = commands.spawn_bundle(self).id();

        commands.entity(parent_entity).add_child(log);
    }
}
