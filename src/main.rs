use bevy::app::AppExit;
use bevy::prelude::*;
use bunner_rs::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// This example illustrates how to use [`States`] to control transitioning from a `Menu` state to
/// an `InGame` state.
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(exit_system))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(movement))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

#[derive(Component)]
struct ButtonExitMarker;

#[derive(Component)]
struct ButtonPlayMarker;

struct MenuData {
    button_entity_exit: Entity,
    button_entity_start: Entity,
}

const NORMAL_BUTTON: Color = Color::ORANGE;
const HOVERED_BUTTON: Color = Color::GREEN;
const PRESSED_BUTTON: Color = Color::RED;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity_start = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // image: UiImage::from(asset_server.load("images/start1.png")),
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ButtonPlayMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/ALGER.TTF"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    let button_entity_exit = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // image: UiImage::from(asset_server.load("images/start0.png")),
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ButtonExitMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/ALGER.TTF"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    commands.insert_resource(MenuData {
        button_entity_start,
        button_entity_exit,
    });
}

fn menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ButtonPlayMarker>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ButtonExitMarker>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.button_entity_start)
        .despawn_recursive();
    commands
        .entity(menu_data.button_entity_exit)
        .despawn_recursive();
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/car31.png"),
        ..Default::default()
    });
}

const SPEED: f32 = 100.0;
fn movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}
