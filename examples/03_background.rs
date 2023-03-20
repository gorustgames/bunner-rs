use bevy::prelude::*;
use bevy::sprite::Anchor;
use bunner_rs::ecs::components::background_row::{RailRow, Row};
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
    let mut row_to_draw = row;

    for i in 0..n {
        if i > 0 {
            row_to_draw = row_to_draw.next();
        }

        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);

        println!("drawing {}", row_to_draw.get_img_name());

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: asset_server.load(&row_to_draw.get_img_name()),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        });
    }
}

#[allow(dead_code)]
fn add_segments(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    count: i8,
    image_base: &str,
    offset_from_bottom: f32,
) {
    for i in 0..count {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);
        println!("y=={}", y);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: asset_server.load(&format!("images/{}{}.png", image_base, i)),
            transform: Transform::from_xyz(x, y, 0.), // see https://bevy-cheatbook.github.io/features/coords.html
            ..default()
        });
    }
}

// https://users.rust-lang.org/t/solved-placement-of-mut-in-function-parameters/19891
#[allow(dead_code)]
fn add_rail(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 4, "rail", offset_from_bottom);
}

#[allow(dead_code)]
fn add_road(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 6, "road", offset_from_bottom);
}

#[allow(dead_code)]
fn add_side(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 3, "side", offset_from_bottom);
}

#[allow(dead_code)]
fn add_water(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 8, "water", offset_from_bottom);
}

#[allow(dead_code)]
fn add_grass(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "grass", offset_from_bottom);
}

#[allow(dead_code)]
fn add_dirt(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "dirt", offset_from_bottom);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // rail+water+pavement
    /*add_rail(&mut commands, &asset_server, 0.); // 4 rails x 40 px
    add_water(&mut commands, &asset_server, 160.); // 8 waters x 40 px
    add_side(&mut commands, &asset_server, 160. + 320.); */

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
    ); /**/
}
