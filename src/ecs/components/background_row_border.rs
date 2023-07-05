use crate::Z_GRID;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameRowBorder;

#[derive(Bundle)]
pub struct GameRowBorderBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    game_row_border: GameRowBorder,
}

impl GameRowBorderBundle {
    pub fn new(y: f32) -> Self {
        let new_bundle = GameRowBorderBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(480., 1.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., y, Z_GRID),
                ..Default::default()
            },
            game_row_border: GameRowBorder,
        };

        new_bundle
    }

    pub fn spawn_bundle(self, commands: &mut Commands) {
        commands.spawn_bundle(self);
    }
}
