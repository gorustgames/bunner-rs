use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, Row, RowType, WaterRowMarker,
};
use bunner_rs::ecs::components::log::{LogBundle, LogSize};
use bunner_rs::ecs::components::player::PlayerBundle;
use bunner_rs::ecs::components::MovementDirection;
use bunner_rs::ecs::resources::BackgroundRows;
use bunner_rs::{
    get_random_i32, is_even_number, LOG_BIG_WIDTH, LOG_SMALL_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    SEGMENT_HEIGHT, SEGMENT_WIDTH,
};
use std::boxed::Box;

#[derive(Component)]
struct DebugText;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_setup)
        .add_system(put_logs_on_water)
        .add_system(text_update_system)
        .insert_resource(BackgroundRows::new())
        .run();
}

/// modified version of game_setup which will make sure
/// we get some water rows after initial grass rows
pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bg_rows: ResMut<BackgroundRows>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    /*
    let offset_from_bottom = 0.;
    let row_count = 20;

    let mut rows: Vec<Box<dyn Row>> = vec![];
    rows.push(Box::new(GrassRow::new_grass_row(0)));

    for i in 0..row_count {
        if i > 0 {
            let mut next_row = rows.get(i as usize - 1).unwrap().next();
            // make sure we get water row after the grass
            if next_row.get_row_type() == RowType::ROAD {
                while next_row.get_row_type() == RowType::ROAD {
                    next_row = rows.get(i as usize - 1).unwrap().next();
                }
            }
            rows.push(next_row);
        }
    }

    rows.reverse();

    for i in 0..row_count {
        let x = -1. * (SCREEN_WIDTH / 2.);
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let row = rows.pop().unwrap();

        bg_rows.add_row(row.clone_row());
        let new_bundle = GameRowBundle::new(row, x, y, &asset_server, i == row_count - 1);
        new_bundle.spawn_bundle_with_markers(&mut commands);
    }

    // center player in the middle of the screen at the last grass
    //  row of bottom grass section (8 grass rows in total)
    let player_x = 0. - SEGMENT_WIDTH / 2.;
    let player_y = -1. * (SCREEN_HEIGHT / 2.) + 8. * SEGMENT_HEIGHT;
    PlayerBundle::new(player_x, player_y, &asset_server, &mut texture_atlas_assets)
        .spawn_player(&mut commands);*/

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Play",
                TextStyle {
                    font: asset_server.load("fonts/ALGER.TTF"),
                    font_size: 40.0,
                    color: Color::RED,
                },
                Default::default(),
            ),
            //transform: Transform::from_xyz(100., 100., 100.),
            ..Default::default()
        })
        .insert(DebugText);
}

fn get_random_log_size() -> LogSize {
    if get_random_i32(1, 2) == 1 {
        LogSize::SMALL
    } else {
        LogSize::BIG
    }
}

pub fn put_logs_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<WaterRowMarker>>,
) {
    const LOGS_PER_ROW: i32 = 10;
    const LOGS_GAP_FROM: i32 = 20;
    const LOGS_GAP_TO: i32 = 250;

    // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
    let mut x_even_row = 0.;
    let mut x_odd_row = SCREEN_WIDTH / 2. - LOG_SMALL_WIDTH as f32;

    for (entity, bg_row) in q.iter_mut() {
        if bg_row.is_water_row {
            for i in 1..LOGS_PER_ROW + 1 {
                // choose big or small randomly
                let log_size = get_random_log_size();

                if is_even_number(bg_row.row.get_index())
                /* even rows*/
                {
                    // handle logs for even rows. these logs are flowing from left to right
                    // choose negative X offset from previous log randomly so that logs do not overlap
                    // the space between two logs will be within range <LOGS_GAP_FROM, LOGS_GAP_TO>
                    if i > 1 {
                        x_even_row = if log_size == LogSize::BIG {
                            x_even_row
                                - get_random_i32(
                                    LOG_BIG_WIDTH + LOGS_GAP_FROM,
                                    LOG_BIG_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        } else {
                            x_even_row
                                - get_random_i32(
                                    LOG_SMALL_WIDTH + LOGS_GAP_FROM,
                                    LOG_SMALL_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        };
                    }

                    LogBundle::new(
                        MovementDirection::LEFT,
                        log_size,
                        x_even_row,
                        0.,
                        &asset_server,
                    )
                    .spawn_log(&mut commands, entity);
                } else
                /* odd rows */
                {
                    // handle logs for odd rows. these logs are flowing from right to left
                    // choose positive X offset from previous log randomly so that logs do not overlap
                    // the space between two logs will be within range <20,200>
                    if i > 1 {
                        x_odd_row = if log_size == LogSize::BIG {
                            x_odd_row
                                + get_random_i32(
                                    LOG_BIG_WIDTH + LOGS_GAP_FROM,
                                    LOG_BIG_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        } else {
                            x_odd_row
                                + get_random_i32(
                                    LOG_SMALL_WIDTH + LOGS_GAP_FROM,
                                    LOG_SMALL_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        };
                    }

                    LogBundle::new(
                        MovementDirection::LEFT,
                        log_size,
                        x_odd_row,
                        0.,
                        &asset_server,
                    )
                    .spawn_log(&mut commands, entity);
                }
            }
        }
    }
}

fn text_update_system(mut q: Query<&mut Text, With<DebugText>>) {
    for mut text in q.iter_mut() {
        //println!("text {}", text.sections[0].value);
        text.sections[0].value = "Play".to_string();
    }
}
