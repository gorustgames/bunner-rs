use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Debug;

mod dirt_row;
mod grass_row;
mod pavement_row;
mod rail_row;
mod road_row;
mod row;
mod water_row;

// reexport underlying structs to make importing shorter
pub use dirt_row::*;
pub use grass_row::*;
pub use pavement_row::*;
pub use rail_row::*;
pub use road_row::*;
pub use row::*;
pub use water_row::*;

fn get_road_or_water_row() -> Box<dyn row::Row> {
    let mut rng = thread_rng();

    if *[0, 1].choose(&mut rng).unwrap() == 1 {
        Box::new(water_row::WaterRow::new_water_row(0))
    } else {
        Box::new(road_row::RoadRow::new_road_row(0))
    }
}

#[derive(Component, Debug)]
pub struct BackgroundRow {
    pub row: Box<dyn row::Row>,
    pub is_top_row: bool,
    pub is_water_row: bool,
    pub is_rail_row: bool,
    pub is_road_row: bool,
    pub is_grass_row: bool,
}

#[derive(Component)]
pub struct WaterRowMarker;

#[derive(Component)]
pub struct RailRowMarker;

#[derive(Component)]
pub struct RoadRowMarker;

#[derive(Component)]
pub struct GrassRowMarker;

#[derive(Bundle)]
pub struct GameRowBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    pub game_row: BackgroundRow,
}

impl GameRowBundle {
    pub fn new(
        row: Box<dyn row::Row>,
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
        is_top_row: bool,
    ) -> Self {
        let is_water_row = row.get_img_base() == "water";

        // for rails there is always just one train for all 4 rail rows
        //  hence we need to mark only one of 4 rail segments. we will mark
        // second one (rail1) since rail sprite is designed in a way it needs to be put
        // 40 pixels above rail0. by marking rail1 as RailRowMarker we can position train
        // to relative Y coordinate = 0 (same as we do for water rows)
        // this has one side effect. train would normally disappear immediately when rail1 scrolls off the screen
        // resulting in unnatural disappearance of the train. because of this we are postponing de-spawning of all
        // rail segments and respective child entities (aka trains)
        let is_rail_row = row.get_img_base() == "rail" && row.get_index() == 1;
        let is_road_row = row.get_img_base() == "road";
        let is_grass_row = row.get_img_base() == "grass";

        // for some reason road will overlap grass segment cutting of its top
        // explicit ordering helps to solve the issue
        let z = if row.get_img_base() != "road" {
            1.
        } else {
            0.5
        };

        let new_bundle = GameRowBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(&row.get_img_name()),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            game_row: BackgroundRow {
                row,
                is_top_row,
                is_water_row,
                is_rail_row,
                is_road_row,
                is_grass_row,
            },
        };

        if is_water_row {
            println!("adding new row {:?}", new_bundle.game_row);
        }

        new_bundle
    }

    /// consumes GameRowBundle and spawns it with or without specific row markers based on row type
    pub fn spawn_bundle_with_markers(self, commands: &mut Commands) {
        if self.game_row.is_water_row {
            commands.spawn_bundle(self).insert(WaterRowMarker);
        } else if self.game_row.is_rail_row {
            commands.spawn_bundle(self).insert(RailRowMarker);
        } else if self.game_row.is_road_row {
            commands.spawn_bundle(self).insert(RoadRowMarker);
        } else if self.game_row.is_grass_row {
            commands.spawn_bundle(self).insert(GrassRowMarker);
        } else {
            commands.spawn_bundle(self);
        }
    }
}
