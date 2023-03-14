use bevy::prelude::*;

const MARKER_SIDE_LENGTH: f32 = 40.0;

// https://www.mikechambers.com/blog/2022/10/29/understanding-the-2d-coordinate-system-in-bevy/
fn main() {
    App::new()
        //Insert a WindowDescriptor to set initial window size and to be
        //able to retrieve its value later on.
        //Note it has to be set before call to add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            width: 400.0,
            height: 400.0,
            title: "Coordinate Example".to_string(),
            ..default()
        })
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

//Bundle to make it a bit easier to set and position markers on the screen
#[derive(Bundle)]
struct MarkerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

//takes a transform specifying its position, and color of the sprite / marker
impl MarkerBundle {
    fn new(transform: Transform, color: Color) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,

                    //widht, height
                    custom_size: Some(Vec2::new(MARKER_SIDE_LENGTH, MARKER_SIDE_LENGTH)),
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                transform,
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands, window: Res<WindowDescriptor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //spawn a bunch of sprites / markers in the center and corners of the window

    //CENTER
    commands.spawn_bundle(MarkerBundle::new(
        Transform::from_xyz(0.0, 0.0, 0.0),
        Color::BLUE,
    ));

    //TOP LEFT
    commands.spawn_bundle(MarkerBundle::new(
        Transform::from_xyz(window.width / -2.0, window.height / 2.0, 0.0),
        Color::GREEN,
    ));

    //BOTTOM LEFT
    commands.spawn_bundle(MarkerBundle::new(
        Transform::from_xyz(
            window.width / -2.0,
            window.height / -2.0 + MARKER_SIDE_LENGTH,
            0.0,
        ),
        Color::RED,
    ));

    //TOP RIGHT
    commands.spawn_bundle(MarkerBundle::new(
        Transform::from_xyz(
            window.width / 2.0 - MARKER_SIDE_LENGTH,
            window.height / 2.0,
            0.0,
        ),
        Color::ORANGE,
    ));

    //BOTTOM RIGHT
    commands.spawn_bundle(MarkerBundle::new(
        Transform::from_xyz(
            window.width / 2.0 - MARKER_SIDE_LENGTH,
            window.height / -2.0 + MARKER_SIDE_LENGTH,
            0.0,
        ),
        Color::PURPLE,
    ));
}
