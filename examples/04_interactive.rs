use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, RailRowMarker, Row, WaterRowMarker,
};
use bunner_rs::ecs::components::log::{LogBundle, LogSize};
use bunner_rs::ecs::components::train::TrainBundle;
use bunner_rs::ecs::components::MovementDirection;
use bunner_rs::{get_random_float, get_random_i32, is_even_number, is_odd_number};
use std::boxed::Box;

const SEGMENT_HEIGHT: f32 = 40.;
const SCREEN_HEIGHT: f32 = 800.;
const SCREEN_WIDTH: f32 = 480.;
const SCROLLING_SPEED_BACKGROUND: f32 = 45.;
const SCROLLING_SPEED_LOGS: f32 = 60.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(background_scrolling)
        .add_system(logs_movement)
        .add_system(put_trains_on_rails)
        .add_system(put_logs_on_water)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let offset_from_bottom = 0.;
    let row_count = 20;

    let mut rows: Vec<Box<dyn Row>> = vec![];
    rows.push(Box::new(GrassRow::new_grass_row(0)));

    for i in 0..row_count {
        if i > 0 {
            rows.push(rows.get(i as usize - 1).unwrap().next())
        }
    }

    rows.reverse();

    for i in 0..row_count {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);
        let row = rows.pop().unwrap();

        let new_bundle = GameRowBundle::new(row, x, y, &asset_server, i == row_count - 1);
        new_bundle.spawn_bundle_with_markers(&mut commands);
    }
}

fn background_scrolling(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Transform, &mut BackgroundRow)>,
) {
    for (entity, mut transform, mut bg_row) in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED_BACKGROUND * time.delta_seconds();

        // if current top row's top Y coord is already below top of the screen (i.e. there is blank space) -> create new top row
        if bg_row.is_top_row && transform.translation.y < SCREEN_HEIGHT / 2. - SEGMENT_HEIGHT {
            bg_row.is_top_row = false; // make current top row as non-top since we are going to create new top level block

            // create new row and position it at the top of current top row
            let x = -1. * (SCREEN_WIDTH / 2.);
            let y = transform.translation.y + SEGMENT_HEIGHT;

            let new_bundle = GameRowBundle::new(bg_row.row.next(), x, y, &asset_server, true);
            new_bundle.spawn_bundle_with_markers(&mut commands);
        }

        // remove entity which has scrolled down bellow screen bottom and is not visible any more
        let y_bellow_bottom = -1. * (SCREEN_HEIGHT / 2.) - SEGMENT_HEIGHT;
        if transform.translation.y < y_bellow_bottom {
            //println!("despawning {:?} {:?}", entity, bg_row);
            commands.entity(entity).despawn_recursive(); // remove background row entity and its children (i.e. logs, trains, cars)
        }
    }
}

fn logs_movement(
    q_parent: Query<(&Transform, &BackgroundRow, &mut Children)>,
    mut q_child: Query<&mut Transform, Without<BackgroundRow>>,
    time: Res<Time>,
) {
    for (_parent_transform, bg_row, children) in q_parent.iter() {
        if bg_row.is_water_row {
            for &child in children.iter() {
                if let Ok(mut child_transform) = q_child.get_mut(child) {
                    // logs in odd rows flow from right to left
                    // logs in even rows flow from left to right
                    if is_odd_number(bg_row.row.get_index()) {
                        child_transform.translation.x -=
                            SCROLLING_SPEED_LOGS * time.delta_seconds();
                    } else {
                        child_transform.translation.x +=
                            SCROLLING_SPEED_LOGS * time.delta_seconds();
                    }
                }
            }
        }
    }
}

fn get_random_log_size() -> LogSize {
    if get_random_i32(1, 2) == 1 {
        LogSize::SMALL
    } else {
        LogSize::BIG
    }
}

fn put_trains_on_rails(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<RailRowMarker>>,
) {
    let mut x: f32;

    // 50:50 chance of traing coming from left or right side
    if get_random_float() < 0.5 {
        x = 0. - 100.;
    } else {
        x = SCREEN_WIDTH / 2. - 100.;
    }

    for (entity, bg_row) in q.iter_mut() {
        if bg_row.is_rail_row {
            TrainBundle::new(MovementDirection::LEFT, x, 0., &asset_server)
                .spawn_train(&mut commands, entity);
        }
    }
}

/// puts logs on newly added water row
/// With<Added<WaterRowMarker>>
/// uses bevy change detection to do it only once
/// we are randomizing log size and putting 10 logs in each row
/// with random distance between them from 20 to 200 pixels.
/// 10 random logs should be enough so that there are still some logs
///  on the water while water row is visible (i.e. it does not scroll of vertically) on the screen
fn put_logs_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<WaterRowMarker>>,
) {
    const LOG_BIG_WIDTH: i32 = 138;
    const LOG_SMALL_WIDTH: i32 = 84;
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
