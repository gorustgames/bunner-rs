use crate::Z_ROW_PLAYER;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerDirection {
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
pub struct PlayerDirectionIndex(usize);

impl Default for PlayerDirectionIndex {
    fn default() -> Self {
        PlayerDirectionIndex(0)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    direction: PlayerDirection,
    direction_idx: PlayerDirectionIndex,
    player: Player,
    animation_timer: AnimationTimer,
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
                transform: Transform::from_xyz(x, y, Z_ROW_PLAYER),
                ..default()
            },
            direction: PlayerDirection::default(),
            direction_idx: PlayerDirectionIndex::default(),
            player: Player,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
        }
    }

    pub fn spawn_player(self, commands: &mut Commands) {
        commands.spawn_bundle(self);
    }

    pub fn change_sprite_icon(
        direction: &PlayerDirection,
        direction_idx: &mut PlayerDirectionIndex,
        sprite: &mut TextureAtlasSprite,
    ) {
        direction_idx.0 = if direction_idx.0 == 0 { 1 } else { 0 };
        match *direction {
            PlayerDirection::Up => sprite.index = 0 + direction_idx.0,
            PlayerDirection::Down => sprite.index = 7 + direction_idx.0,
            PlayerDirection::Left => sprite.index = 9 + direction_idx.0,
            PlayerDirection::Right => sprite.index = 3 + direction_idx.0,
        }
    }
}
