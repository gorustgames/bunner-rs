use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, Row, WaterRowMarker,
};
use bunner_rs::ecs::components::MovementDirection;
use bunner_rs::ecs::components::log::{LogBundle, LogSize};
use std::boxed::Box;

const SEGMENT_HEIGHT: f32 = 40.;
const SCREEN_HEIGHT: f32 = 800.;
const SCREEN_WIDTH: f32 = 480.;
const SCROLLING_SPEED: f32 = 45.;

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
        .add_system(children_movement)
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
        if new_bundle.game_row.is_water_row {
            commands.spawn_bundle(new_bundle).insert(WaterRowMarker);
        } else {
            commands.spawn_bundle(new_bundle);
        }
    }
}

fn background_scrolling(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Transform, &mut BackgroundRow)>,
) {
    for (entity, mut transform, mut bg_row) in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED * time.delta_seconds();

        // if current top row's top Y coord is already below top of the screen (i.e. there is blank space) -> create new top row
        if bg_row.is_top_row && transform.translation.y < SCREEN_HEIGHT / 2. - SEGMENT_HEIGHT {
            bg_row.is_top_row = false; // make current top row as non-top since we are going to create new top level block

            // create new row and position it at the top of current top row
            let x = -1. * (SCREEN_WIDTH / 2.);
            let y = transform.translation.y + SEGMENT_HEIGHT;

            let new_bundle = GameRowBundle::new(bg_row.row.next(), x, y, &asset_server, true);
            if new_bundle.game_row.is_water_row {
                commands.spawn_bundle(new_bundle).insert(WaterRowMarker);
            } else {
                commands.spawn_bundle(new_bundle);
            }
        }

        // remove entity which has scrolled down bellow screen bottom and is not visible any more
        let y_bellow_bottom = -1. * (SCREEN_HEIGHT / 2.) - SEGMENT_HEIGHT;
        if transform.translation.y < y_bellow_bottom {
            //println!("despawning {:?} {:?}", entity, bg_row);
            commands.entity(entity).despawn_recursive(); // remove background row entity and its children (i.e. logs, trains, cars)
        }
    }
}

fn children_movement(
    q_parent: Query<(&Transform, &BackgroundRow, &mut Children)>,
    mut q_child: Query<&mut Transform, Without<BackgroundRow>>,
    time: Res<Time>,
) {
    for (_parent_transform, bg_row, children) in q_parent.iter() {
        if bg_row.is_water_row {
            for &child in children.iter() {
                if let Ok(mut child_transform) = q_child.get_mut(child) {
                    child_transform.translation.x += SCROLLING_SPEED * time.delta_seconds();
                }
            }
        }
    }
}

/// puts new log on newly added water row
/// With<Added<WaterRowMarker>>
/// uses bevy change detection to do it only once
fn put_logs_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &Transform, &BackgroundRow), Added<WaterRowMarker>>,
) {
    const LOG_BIG_WIDTH: f32 = 138.;
    const LOG_SMALL_WIDTH: f32 = 84.;

    // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
    let mut x = 0.;
    let mut y = 0.;

    for (entity, _transform, bg_row) in q.iter_mut() {
        if bg_row.is_water_row {
            for n in 1..11 {
                let log = commands
                    .spawn_bundle(LogBundle::new(
                        MovementDirection::LEFT,
                        LogSize::BIG,
                        x,
                        y,
                        &asset_server,
                    ))
                    .id();

                commands.entity(entity).add_child(log);
            }
        }
    }
}
