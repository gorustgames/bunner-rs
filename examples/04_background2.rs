use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{GameRowBundle, RailRow, Row};
use std::boxed::Box;

const SEGMENT_HEIGHT: f32 = 40.;
const SCREEN_HEIGHT: f32 = 800.;
const SCREEN_WIDTH: f32 = 480.;

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
    let mut next_row = row;

    for i in 0..n {
        if i > 0 {
            next_row = next_row.next();
        }
        rows.push(next_row.next())
    }

    rows.reverse();

    for i in 0..n {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);
        let row = rows.pop().unwrap();

        println!("drawing {}", row.get_img_name());

        commands.spawn_bundle(GameRowBundle::new(row, x, y, asset_server));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let rail_count = 4;
    let water_count = 8;
    let sidewalk_count = 3;
    let road_count = 3;
    let water_count_2 = 2; // we have place just for 2 blocks. together we have 20 x 40 px = 800 px which is screen height

    draw_n_rows(
        &mut commands,
        &asset_server,
        Box::new(RailRow::new_rail_row(0)),
        rail_count + water_count + sidewalk_count + road_count + water_count_2,
        0.,
    );
}
