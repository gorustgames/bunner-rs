use crate::Z_SPLASH;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component)]
pub struct Splash {
    animation_index: usize,
    has_splashed: bool,
}

impl Default for Splash {
    fn default() -> Self {
        Splash {
            animation_index: 0,
            has_splashed: false,
        }
    }
}

impl Splash {
    pub fn has_splashed(&self) -> bool {
        self.has_splashed
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Bundle)]
pub struct SplashBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    splash: Splash,
    animation_timer: AnimationTimer,
}

impl SplashBundle {
    pub fn new(
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
        texture_atlas_assets: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let player = asset_server.load("images/splash.png");
        let texture_atlas = TextureAtlas::from_grid(player, Vec2::new(120.0, 84.0), 8, 1);
        let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

        SplashBundle {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(x, y, Z_SPLASH),
                ..default()
            },
            splash: Splash::default(),
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
        }
    }

    pub fn spawn_splash(self, commands: &mut Commands) {
        commands.spawn_bundle(self);
    }

    pub fn change_sprite_icon(splash: &mut Splash, sprite: &mut TextureAtlasSprite) {
        splash.animation_index += 1;
        if splash.animation_index > 7 {
            splash.animation_index = 0;
            splash.has_splashed = true;
        }

        sprite.index = splash.animation_index;
    }
}
