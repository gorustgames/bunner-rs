use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: 480.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct PlayerDirectionIndex(usize);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player = asset_server.load("images/player.png");
    let texture_atlas = TextureAtlas::from_grid(player, Vec2::new(60.0, 60.0), 12, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., 0., 0.), // see https://bevy-cheatbook.github.io/features/coords.html
            ..default()
        })
        .insert(Player)
        .insert(PlayerDirection::Up)
        .insert(PlayerDirectionIndex(0))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

// https://github.com/bevyengine/bevy/discussions/2892
// https://codeshack.io/images-sprite-sheet-generator/
fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &mut PlayerDirection,
            &mut PlayerDirectionIndex,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
) {
    #[allow(unused_variables)]
    if let Ok((
        mut transform,
        mut sprite,
        mut timer,
        mut direction,
        mut direction_idx,
        texture_atlas_handle,
    )) = query.get_single_mut()
    {
        timer.tick(time.delta());

        if keyboard_input.pressed(KeyCode::Up) {
            *direction = PlayerDirection::Up;
            transform.translation.y += 150. * time.delta_seconds();
            if transform.translation.y > 370. {
                transform.translation.y = 370.;
            }

            if timer.just_finished() {
                direction_idx.0 = if direction_idx.0 == 0 { 1 } else { 0 };
                match *direction {
                    PlayerDirection::Up => sprite.index = 0 + direction_idx.0,
                    PlayerDirection::Down => sprite.index = 7 + direction_idx.0,
                    PlayerDirection::Left => sprite.index = 9 + direction_idx.0,
                    PlayerDirection::Right => sprite.index = 3 + direction_idx.0,
                }
            }
            // println!("going up. y {}", transform.translation.y);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *direction = PlayerDirection::Down;
            transform.translation.y -= 150. * time.delta_seconds();
            if transform.translation.y < -370. {
                transform.translation.y = -370.;
            }

            if timer.just_finished() {
                direction_idx.0 = if direction_idx.0 == 0 { 1 } else { 0 };
                match *direction {
                    PlayerDirection::Up => sprite.index = 0 + direction_idx.0,
                    PlayerDirection::Down => sprite.index = 7 + direction_idx.0,
                    PlayerDirection::Left => sprite.index = 9 + direction_idx.0,
                    PlayerDirection::Right => sprite.index = 3 + direction_idx.0,
                }
            }
            // println!("going down. y {}", transform.translation.y);
        }

        if keyboard_input.pressed(KeyCode::Left) {
            *direction = PlayerDirection::Left;
            transform.translation.x -= 150. * time.delta_seconds();
            if transform.translation.x < -220. {
                transform.translation.x = -220.;
            }

            if timer.just_finished() {
                direction_idx.0 = if direction_idx.0 == 0 { 1 } else { 0 };
                match *direction {
                    PlayerDirection::Up => sprite.index = 0 + direction_idx.0,
                    PlayerDirection::Down => sprite.index = 7 + direction_idx.0,
                    PlayerDirection::Left => sprite.index = 9 + direction_idx.0,
                    PlayerDirection::Right => sprite.index = 3 + direction_idx.0,
                }
            }
            // println!("going left. x {}", transform.translation.x);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *direction = PlayerDirection::Right;
            transform.translation.x += 150. * time.delta_seconds();
            if transform.translation.x > 220. {
                transform.translation.x = 220.;
            }

            if timer.just_finished() {
                direction_idx.0 = if direction_idx.0 == 0 { 1 } else { 0 };
                match *direction {
                    PlayerDirection::Up => sprite.index = 0 + direction_idx.0,
                    PlayerDirection::Down => sprite.index = 7 + direction_idx.0,
                    PlayerDirection::Left => sprite.index = 9 + direction_idx.0,
                    PlayerDirection::Right => sprite.index = 3 + direction_idx.0,
                }
            }
            // println!("going right. x {}", transform.translation.x);
        }
    }
}
