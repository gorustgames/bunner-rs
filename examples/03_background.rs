use bevy::prelude::*;
use bevy::sprite::Anchor;
use bunner_rs::ecs::components::bush::{BushBundle, BushHorizontalType, BushVerticalType};
use bunner_rs::ecs::components::car::CarBundle;
use bunner_rs::ecs::components::log::{LogBundle, LogSize};
use bunner_rs::ecs::components::train::TrainBundle;
use bunner_rs::ecs::components::MovementDirection;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
        .add_startup_system(setup_bushes)
        .run();
}

fn get_road_or_water_row() -> Box<dyn Row> {
    let mut rng = thread_rng();

    if *[0, 1].choose(&mut rng).unwrap() == 1 {
        Box::new(WaterRow::new_water_row(0))
    } else {
        Box::new(RoadRow::new_road_row(0))
    }
}

trait Row {
    fn next(&self) -> Box<dyn Row>;
    fn get_index(&self) -> i8;
    fn get_img_base(&self) -> String;
    fn get_img_name(&self) -> String {
        format!("images/{}{}.png", self.get_img_base(), self.get_index())
    }
}

/// rail
struct RailRow {
    index: i8,
}

impl RailRow {
    pub fn new_rail_row(index: i8) -> Self {
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
    pub fn new_water_row(index: i8) -> Self {
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
    pub fn new_pavement_row(index: i8) -> Self {
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
    pub fn new_road_row(index: i8) -> Self {
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
    pub fn new_dirt_row(index: i8) -> Self {
        DirtRow { index }
    }
}

impl Row for DirtRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            0..=5 => Box::new(DirtRow::new_dirt_row(self.index + 8)),
            6 => Box::new(DirtRow::new_dirt_row(7)),
            7 => Box::new(DirtRow::new_dirt_row(15)),
            8..=14 => Box::new(DirtRow::new_dirt_row(self.index + 1)),
            _ => get_road_or_water_row(),
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
    pub fn new_grass_row(index: i8) -> Self {
        GrassRow { index }
    }
}

impl Row for GrassRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            // match inclusive range
            0..=5 => Box::new(GrassRow::new_grass_row(self.index + 8)),
            6 => Box::new(GrassRow::new_grass_row(7)),
            7 => Box::new(GrassRow::new_grass_row(15)),
            8..=14 => Box::new(GrassRow::new_grass_row(self.index + 1)),
            _ => get_road_or_water_row(),
        }
    }
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "grass".to_string()
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

        // for some reason road will overlap grass segment cutting of its top
        // explicit ordering helps to solve the issue
        let z = if image_base != "road" { 1. } else { 0.5 };
        println!("y={}", y);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: asset_server.load(&format!("images/{}{}.png", image_base, i)),
            transform: Transform::from_xyz(x, y, z), // see https://bevy-cheatbook.github.io/features/coords.html
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
    add_segments(commands, asset_server, 2, "road", offset_from_bottom);
}

#[allow(dead_code)]
fn add_side(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 3, "side", offset_from_bottom);
}

#[allow(dead_code)]
fn add_water(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 2, "water", offset_from_bottom);
}

#[allow(dead_code)]
fn add_grass(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "grass", offset_from_bottom);
}

#[allow(dead_code)]
fn add_dirt(commands: &mut Commands, asset_server: &Res<AssetServer>, offset_from_bottom: f32) {
    add_segments(commands, asset_server, 15, "dirt", offset_from_bottom);
}

#[allow(dead_code)]
fn setup_hardcoded(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    add_rail(&mut commands, &asset_server, 0.); // 4 rails x 40 px
    add_road(&mut commands, &asset_server, 4. * 40.); // 2 road x 40
    add_rail(&mut commands, &asset_server, 4. * 40. + 2. * 40.); // 4 rails x 40 px
    add_water(&mut commands, &asset_server, 4. * 40. + 2. * 40. + 4. * 40.); // 2 waters x 40 px
    add_side(
        &mut commands,
        &asset_server,
        4. * 40. + 2. * 40. + 4. * 40. + 2. * 40.,
    ); // 3 sides x 40 px
    add_road(
        &mut commands,
        &asset_server,
        4. * 40. + 2. * 40. + 4. * 40. + 2. * 40. + 3. * 40.,
    ); // 2 road x 40
}

#[allow(dead_code)]
fn setup_bushes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    draw_n_rows(
        &mut commands,
        &asset_server,
        Box::new(GrassRow::new_grass_row(0)),
        10,
        0.,
    );

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::LEFTMOST,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::LEFTMOST,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 1.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::MIDDLE1,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 1.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::MIDDLE1,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 2.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::MIDDLE2,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 2.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::MIDDLE2,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 3.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::MIDDLE3,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 3.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::MIDDLE3,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 4.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::RIGHTMOST,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 4.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::RIGHTMOST,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 6.,
        -360.,
        BushVerticalType::BOTTOM,
        BushHorizontalType::SINGLE,
    ));

    commands.spawn_bundle(BushBundle::new(
        &asset_server,
        -200. + 40. * 6.,
        -320.,
        BushVerticalType::TOP,
        BushHorizontalType::SINGLE,
    ));
}

#[allow(dead_code)]
fn setup_random(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.spawn_bundle(LogBundle::new(
        MovementDirection::LEFT,
        LogSize::BIG,
        -240.,
        -400. + 160.,
        &asset_server,
    ));

    commands.spawn_bundle(LogBundle::new(
        MovementDirection::LEFT,
        LogSize::BIG,
        -240. + 138., // big log is 138x60 px
        -400. + 200.,
        &asset_server,
    ));

    commands.spawn_bundle(TrainBundle::new(
        MovementDirection::LEFT,
        -240. + 100., // train is 860x134 px
        -400. + 40., // train looks good when its bottom is aligned to rail1.png row, i.e. offset 40px representing rail0.jpg
        &asset_server,
    ));

    commands.spawn_bundle(CarBundle::new(
        MovementDirection::LEFT,
        240. - 90.,        // car is 90x59 px
        -400. + 15. * 40., // there are 15 rows till the first road row
        200.,
        &asset_server,
    ));

    commands.spawn_bundle(CarBundle::new(
        MovementDirection::LEFT,
        240. - 90.,
        -400. + 16. * 40.,
        200.,
        &asset_server,
    ));

    commands.spawn_bundle(CarBundle::new(
        MovementDirection::RIGHT,
        240. - 90.,
        -400. + 17. * 40.,
        200.,
        &asset_server,
    ));
}
