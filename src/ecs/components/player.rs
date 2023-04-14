use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component)]
struct Player;

#[derive(Component)]
enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        PlayerDirection::Up
    }
}

/// PlayerDirectionIndex can be 0, 1 or 2
/// 0 & 1 are indices of normal movement in given direction
/// 2 is index used when rabbit is hit by car or train (while running in given direction)
#[derive(Component)]
struct PlayerDirectionIndex(usize);

impl Default for PlayerDirectionIndex {
    fn default() -> Self {
        PlayerDirectionIndex(0)
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    direction: PlayerDirection,
    direction_idx: PlayerDirectionIndex,
}

impl PlayerBundle {
    pub fn new(
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
        texture_atlas_assets: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let player = asset_server.load("images/player.png");
        let texture_atlas = TextureAtlas::from_grid(player, Vec2::new(60.0, 60.0), 12, 1);
        let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(x, y, 1.),
                ..default()
            },
            direction: PlayerDirection::default(),
            direction_idx: PlayerDirectionIndex::default(),
        }
    }

    pub fn spawn_player(self, commands: &mut Commands) {
        commands.spawn_bundle(self);
    }
}
