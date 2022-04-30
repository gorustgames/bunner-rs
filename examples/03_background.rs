use bevy::prelude::*;
use std::boxed::Box;

const SEGMENT_HEIGHT: f32 = 40.;
const SCREEN_HEIGHT: f32 = 800.;
const SCREEN_WIDTH: f32 = 480.;
#[allow(dead_code)]
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

trait Row {
    fn next(&self) -> Box<dyn Row>;
    fn get_index(&self) -> i8;
    fn get_img_base(&self) -> String;
    fn get_img_name(&self) -> String {
        format!("images/{}{}.png", self.get_img_base(), self.get_index())
    }
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

        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32 + 1.) - SEGMENT_HEIGHT / 2.
            + offset_from_bottom;

        println!("drawing {}", row_to_draw.get_img_name());

        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(&row_to_draw.get_img_name()),
            transform: Transform::from_xyz(0., y, 0.),
            ..default()
        });
    }
}

/// rail
struct RailRow {
    index: i8,
}

impl RailRow {
    fn new_rail_row(index: i8) -> Self {
        RailRow { index }
    }
}

impl Row for RailRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 3 {
            Box::new(RailRow::new_rail_row(self.index + 1))
        } else {
            Box::new(WaterRow::new_water_row(0))
        }
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "rail".to_string()
    }
}

/// water
struct WaterRow {
    index: i8,
}

impl WaterRow {
    fn new_water_row(index: i8) -> Self {
        WaterRow { index }
    }
}

impl Row for WaterRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 7 {
            Box::new(WaterRow::new_water_row(self.index + 1))
        } else {
            Box::new(PavementRow::new_pavement_row(0))
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "water".to_string()
    }
}

/// pavement
struct PavementRow {
    index: i8,
}

impl PavementRow {
    fn new_pavement_row(index: i8) -> Self {
        PavementRow { index }
    }
}

impl Row for PavementRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 2 {
            Box::new(PavementRow::new_pavement_row(self.index + 1))
        } else {
            Box::new(RoadRow::new_road_row(0))
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "side".to_string()
    }
}

/// road
struct RoadRow {
    index: i8,
}

impl RoadRow {
    fn new_road_row(index: i8) -> Self {
        RoadRow { index }
    }
}

impl Row for RoadRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 2 {
            Box::new(RoadRow::new_road_row(self.index + 1))
        } else {
            Box::new(WaterRow::new_water_row(0))
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "road".to_string()
    }
}

/// dirt
struct DirtRow {
    index: i8,
}

impl DirtRow {
    fn new_dirt_row(index: i8) -> Self {
        DirtRow { index }
    }
}

impl Row for DirtRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            1..=5 => Box::new(DirtRow::new_dirt_row(self.index + 8)),
            6 => Box::new(DirtRow::new_dirt_row(7)),
            7 => Box::new(DirtRow::new_dirt_row(15)),
            8..=14 => Box::new(DirtRow::new_dirt_row(self.index + 1)),
            _ => Box::new(WaterRow::new_water_row(0)),
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "dirt".to_string()
    }
}

///grass
struct GrassRow {
    index: i8,
}

impl GrassRow {
    fn new_grass_row(index: i8) -> Self {
        GrassRow { index }
    }
}

impl Row for GrassRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            1..=5 => Box::new(GrassRow::new_grass_row(self.index + 8)),
            6 => Box::new(GrassRow::new_grass_row(7)),
            7 => Box::new(GrassRow::new_grass_row(15)),
            8..=14 => Box::new(GrassRow::new_grass_row(self.index + 1)),
            _ => Box::new(WaterRow::new_water_row(0)),
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "grass".to_string()
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
    /*add_rail(&mut commands, &asset_server, 0.);
    add_water(&mut commands, &asset_server, 160. - SEGMENT_OVERLAP_OFFSET);
    add_side(&mut commands, &asset_server, 160. + 320.- SEGMENT_OVERLAP_OFFSET);*/

    draw_n_rows(
        &mut commands,
        &asset_server,
        Box::new(RailRow::new_rail_row(0)),
        4 + 8 + 3 + 4,
        0.,
    );
}
