use bevy::prelude::*;
use bunner_rs::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, GrassRowMarker, Row, RowType,
};

use bunner_rs::ecs::components::bush::{BushBundle, BushHorizontalType, BushVerticalType};
use bunner_rs::ecs::resources::BackgroundRows;
use bunner_rs::{SCREEN_HEIGHT, SCREEN_WIDTH, SEGMENT_HEIGHT, SEGMENT_WIDTH};
use std::boxed::Box;

/// This sample was created to test bushes generative algorithm
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(put_bushes_on_grass)
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

    let dummy_random_mask: [bool; 12] = [
        false, false, true, false, true, false, true, false, false, true, true, false,
    ];

    for i in 0..row_count {
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let x = -1. * (SCREEN_WIDTH / 2.);
        let mut row = rows.pop().unwrap();

        generate_hedge(&mut row, &bg_rows, dummy_random_mask);

        println!("adding row {:?}", row);

        bg_rows.add_row(row.clone_row());
        let new_bundle = GameRowBundle::new(row, x, y, &asset_server, i == row_count - 1);
        new_bundle.spawn_bundle_with_markers(&mut commands);
    }
}

fn generate_hedge(
    next_bg_row: &mut Box<dyn Row>,
    bg_rows: &ResMut<BackgroundRows>,
    input_mask: [bool; 12],
) {
    if next_bg_row.get_row_type() == RowType::GRASS {
        // generate bushes only for certain grass rows (7,14)
        let is_mask_eligible = next_bg_row.get_index() > 7 && next_bg_row.get_index() < 14;

        if let Some(previous_row) = bg_rows.last_row() {
            if let Some(row_mask) = previous_row.get_row_mask() {
                if let Some(row_data) = previous_row.get_row_data() {
                    if let Ok(row_with_top_bushes) = row_data.downcast::<bool>() {
                        if !*row_with_top_bushes {
                            // create top hedge row only if previous row is bottom row
                            // if it is top row do not create anything, we want have gap
                            // between two hedges
                            next_bg_row.set_row_mask(row_mask);
                            next_bg_row.set_row_data(Box::new(true)); // this is top hedge row
                        }
                    }
                }
            } else {
                if is_mask_eligible {
                    next_bg_row.set_row_mask(input_mask);
                    next_bg_row.set_row_data(Box::new(false)); // this is bottom hedge row
                }
            }
        }
    }
}

pub fn put_bushes_on_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<GrassRowMarker>>,
) {
    for (entity, bg_row) in q.iter_mut() {
        if bg_row.row.get_row_type() == RowType::GRASS {
            if let Some(mask) = bg_row.row.get_row_mask() {
                println!("processing row {:?}", bg_row);
                let mut bush_vertical_type = BushVerticalType::BOTTOM;
                let mut bush_horizontal_type;

                if let Ok(row_with_top_bushes) =
                    bg_row.row.get_row_data().unwrap().downcast::<bool>()
                {
                    bush_vertical_type = if *row_with_top_bushes == true {
                        BushVerticalType::TOP
                    } else {
                        BushVerticalType::BOTTOM
                    };
                }

                for i in 0..12 {
                    if i == 0 {
                        if mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::SINGLE;
                        } else {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        }
                    } else if i > 0 && i < 11 {
                        if mask[i - 1] == true && mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::SINGLE;
                        } else if mask[i - 1] == true && mask[i + 1] == false {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        } else if mask[i - 1] == false && mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::RIGHTMOST;
                        } else {
                            bush_horizontal_type = BushHorizontalType::MIDDLE1;
                        }
                    } else {
                        if mask[i - 1] == true {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        } else {
                            bush_horizontal_type = BushHorizontalType::MIDDLE1;
                        }
                    }

                    if !mask[i]
                    /* if there should be a bush */
                    {
                        println!(
                            "adding bush {:?} - vt: {:?} ht: {:?}",
                            bg_row, bush_vertical_type, bush_horizontal_type
                        );
                        let bush_bundle = BushBundle::new(
                            &asset_server,
                            0. + i as f32 * SEGMENT_WIDTH,
                            0.,
                            bush_vertical_type,
                            bush_horizontal_type,
                        );

                        let bush = commands.spawn_bundle(bush_bundle).id();
                        commands.entity(entity).add_child(bush);
                    }
                }
            }
        }
    }
}
