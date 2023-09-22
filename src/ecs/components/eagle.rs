use crate::ecs::components::EagleTimer;
use crate::{SCREEN_HEIGHT, Z_ROW_CHILD_COMPONENT_EAGLE};
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Bundle)]
pub struct EagleBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl EagleBundle {
    pub fn new(x: f32, asset_server: &Res<AssetServer>) -> Self {
        EagleBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load("images/eagle.png"),
                transform: Transform::from_xyz(x, SCREEN_HEIGHT / 2., Z_ROW_CHILD_COMPONENT_EAGLE),
                ..default()
            },
        }
    }

    /// spawns train bundle with delay
    pub fn spawn_eagle_with_delay(self, commands: &mut Commands, delay_sec: f32) {
        commands
            .spawn_bundle(self)
            .insert(EagleTimer::new(delay_sec));
    }
}
