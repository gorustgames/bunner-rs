use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{GameRowBundle, GrassRow, Row};

use bunner_rs::ecs::resources::BackgroundRows;
use bunner_rs::ecs::systems::*;
use bunner_rs::ecs::systems::{delayed_despawn_recursive, delayed_spawn_train};
use bunner_rs::{SCREEN_HEIGHT, SCREEN_WIDTH, SEGMENT_HEIGHT};
use std::boxed::Box;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(background_scrolling)
        .add_system(put_trains_on_rails)
        .add_system(put_logs_on_water)
        .add_system(put_bushes_on_grass)
        .add_system(put_cars_on_roads)
        .add_system(logs_movement)
        .add_system(trains_movement)
        .add_system(cars_movement)
        .add_system(delayed_despawn_recursive)
        .add_system(delayed_spawn_train)
        .add_system(delayed_spawn_car)
        .insert_resource(BackgroundRows::new())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bg_rows: ResMut<BackgroundRows>,
) {
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

        bg_rows.add_row(row.clone_row());
        let new_bundle = GameRowBundle::new(row, x, y, &asset_server, i == row_count - 1);
        new_bundle.spawn_bundle_with_markers(&mut commands);
    }
}
