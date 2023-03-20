use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Debug;

pub trait Row: Send + Sync + Debug {
    fn next(&self) -> Box<dyn Row>;
    fn get_index(&self) -> i8;
    fn get_img_base(&self) -> String;
    fn get_img_name(&self) -> String {
        format!("images/{}{}.png", self.get_img_base(), self.get_index())
    }
}

fn get_road_or_water_row() -> Box<dyn Row> {
    let mut rng = thread_rng();

    if *[0, 1].choose(&mut rng).unwrap() == 1 {
        Box::new(WaterRow::new_water_row(0))
    } else {
        Box::new(RoadRow::new_road_row(0))
    }
}

/// rail
#[derive(Debug)]
pub struct RailRow {
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
#[derive(Debug)]
pub struct WaterRow {
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
#[derive(Debug)]
pub struct PavementRow {
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
#[derive(Debug)]
pub struct RoadRow {
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
#[derive(Debug)]
pub struct DirtRow {
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
#[derive(Debug)]
pub struct GrassRow {
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

#[derive(Component, Debug)]
pub struct BackgroundRow {
    pub row: Box<dyn Row>,
    pub is_top_row: bool,
}

#[derive(Bundle)]
pub struct GameRowBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    game_row: BackgroundRow,
}

impl GameRowBundle {
    pub fn new(
        row: Box<dyn Row>,
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
        is_top_row: bool,
    ) -> Self {
        GameRowBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(&row.get_img_name()),
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            },
            game_row: BackgroundRow { row, is_top_row },
        }
    }
}
