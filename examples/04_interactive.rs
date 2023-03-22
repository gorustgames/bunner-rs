use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{BackgroundRow, GameRowBundle, GrassRow, Row};
use bunner_rs::ecs::components::log::{LogBundle, LogSize};
use bunner_rs::MovementDirection;
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
        .add_system(children_scrolling)
        .add_system(put_log_on_water)
        .run();
}

fn draw_n_rows(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    row: Box<dyn Row>,
    n: i8,
    offset_from_bottom: f32,
) {
    let mut rows = vec![];
    rows.push(row);

    for i in 0..n {
        if i > 0 {
            rows.push(rows.get(i as usize - 1).unwrap().next())
        }
    }

    rows.reverse();

    for i in 0..n {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);
        let row = rows.pop().unwrap();

        commands.spawn_bundle(GameRowBundle::new(row, x, y, asset_server, i == n - 1));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    draw_n_rows(
        &mut commands,
        &asset_server,
        Box::new(GrassRow::new_grass_row(0)),
        20, // draw 20 rows x 40 px = 800 px i.e. populate whole screen initially
        0.,
    );
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
        if bg_row.is_top_row && transform.translation.y < SCREEN_HEIGHT - SEGMENT_HEIGHT {
            bg_row.is_top_row = false; // make current top row as non-top since we are going to create new top level block

            // create new row and position it at the top of current top row
            let x = -1. * (SCREEN_WIDTH / 2.);
            let y = transform.translation.y + SEGMENT_HEIGHT;
            commands.spawn_bundle(GameRowBundle::new(
                bg_row.row.next(),
                x,
                y,
                &asset_server,
                true,
            ));
        }

        // remove entity which has scrolled down bellow screen bottom and is not visible any more
        let y_bellow_bottom = -1. * (SCREEN_HEIGHT / 2.) - SEGMENT_HEIGHT;
        if transform.translation.y < y_bellow_bottom {
            println!("despawning {:?} {:?}", entity, bg_row);
            commands.entity(entity).despawn_recursive(); // remove background row entity and its children (i.e. logs, trains, cars)
        }
    }
}

fn children_scrolling(
    q_parent: Query<(&Transform, &BackgroundRow, &mut Children)>,
    mut q_child: Query<&mut Transform, Without<BackgroundRow>>,
) {
    for (parent_transform, bg_row, children) in q_parent.iter() {
        if bg_row.row.get_img_base() == "water" {
            for &child in children.iter() {
                if let Ok(mut child_transform) = q_child.get_mut(child) {
                    child_transform.translation.y = parent_transform.translation.y;
                }
            }
        }
    }
}

fn put_log_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _time: Res<Time>,
    mut q: Query<(Entity, &Transform, &BackgroundRow)>,
) {
    for (entity, transform, bg_row) in q.iter_mut() {
        if bg_row.row.get_img_base() == "water" {
            let x = 0.;
            let y = transform.translation.y;

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
