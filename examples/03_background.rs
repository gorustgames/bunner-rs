use bevy::prelude::*;

const SEGMENT_HEIGHT: f32 = 40.;
const SCREEN_HEIGHT: f32 = 800.;
const SCREEN_WIDTH: f32 = 480.;
const SEGMENT_OVERLAP_OFFSET: f32 = 5.;

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


trait RowSegment {
    fn next(&self) -> Self;
}

/// rail
struct RailRow {
    index: i8,
}

impl RailRow {
    fn new_rail_row(index: i8) -> Self {
        RailRow {
            index,
        }
    }
}

impl RowSegment for RailRow {
    fn next(&self) -> Self {
        RailRow::new_rail_row(1)
    }
}

/// pavement
struct PavementRow {
    index: i8,
}

impl PavementRow {
    fn new_pavement_row(index: i8) -> Self {
        PavementRow {
            index,
        }
    }
}

impl RowSegment for PavementRow {
    fn next(&self) -> Self {
        PavementRow::new_pavement_row(1)
    }
}

fn add_segments(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    count: i8,
    image_base: &str,
    offset_from_bottom: f32,
) {
    for i in 0..count {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32 + 1.) - SEGMENT_HEIGHT / 2.
            + offset_from_bottom;
        println!("y=={}", y);
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(&format!("images/{}{}.png", image_base, i)),
            transform: Transform::from_xyz(0., y, 0.), // see https://bevy-cheatbook.github.io/features/coords.html
            ..default()
        });
    }
}

fn add_segment(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    segment_index: i8,
    image_base: &str,
    offset_from_bottom: f32,
) {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT - SEGMENT_HEIGHT / 2.
            + offset_from_bottom;
        println!("y=={}", y);
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(&format!("images/{}{}.png", image_base, segment_index)),
            transform: Transform::from_xyz(0., y, 0.),
            ..default()
        });

}

// https://users.rust-lang.org/t/solved-placement-of-mut-in-function-parameters/19891
fn add_rail(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 4, "rail", offset_from_bottom);
}

fn add_road(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 6, "road", offset_from_bottom);
}

fn add_side(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 3, "side", offset_from_bottom);
}

fn add_water(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 8, "water", offset_from_bottom);
}

fn add_grass(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "grass", offset_from_bottom);
}

fn add_dirt(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "dirt", offset_from_bottom);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // rail+water+pavement
    /*add_rail(&mut commands, &asset_server, 0.);
    add_water(&mut commands, &asset_server, 160. - SEGMENT_OVERLAP_OFFSET);
    add_side(&mut commands, &asset_server, 160. + 320.- SEGMENT_OVERLAP_OFFSET);*/


    // just the road
    // add_road(&mut commands, &asset_server, 0.);

    // some grass
    add_segment(&mut commands, &asset_server, 15, "grass", SEGMENT_OVERLAP_OFFSET);

    //add_dirt(&mut commands, &asset_server, 0.);

    //
    //

}
